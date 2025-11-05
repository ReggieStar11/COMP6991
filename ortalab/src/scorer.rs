use crate::cards::PlayedCard;
use crate::jokers::{self, JokerEffect};
use ortalib::{Card, Chips, Edition, Enhancement, JokerCard, Mult, PokerHand, Rank, Round, Suit};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScoringState {
    pub round: Round,
    pub chips: Chips,
    pub mult: Mult,
    pub _explain: Vec<String>,
}

impl ScoringState {
    pub fn new(round: Round) -> Self {
        Self {
            round,
            chips: 0.0,
            mult: 1.0,
            _explain: Vec::new(),
        }
    }

    pub fn _lowest_held_rank_value(&self) -> Option<Chips> {
        self.round
            .cards_held_in_hand
            .iter()
            .map(|c| c.rank.rank_value())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn find_lowest_rightmost_held_card(&self) -> Option<PlayedCard> {
        self.round
            .cards_held_in_hand
            .iter()
            .enumerate()
            .map(|(idx, card)| (PlayedCard::new(*card), idx))
            .min_by(|(pc1, idx1), (pc2, idx2)| {
                pc1.inner
                    .rank
                    .rank_value()
                    .partial_cmp(&pc2.inner.rank.rank_value())
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| idx2.cmp(idx1)) // Rightmost in case of tie
            })
            .map(|(pc, _)| pc)
    }
}

pub struct ScoringEngine {
    state: ScoringState,
    joker_registry: std::collections::HashMap<ortalib::Joker, Box<dyn JokerEffect>>,
    best_poker_hand: (PokerHand, Vec<Card>),
    pareidolia_active: bool,
    splash_active: bool,
}

impl ScoringEngine {
    pub fn new(round: Round) -> Self {
        let state = ScoringState::new(round);
        let joker_registry = jokers::build_registry();

        let mut pareidolia_active = false;
        let mut splash_active = false;
        let mut four_fingers_active = false;
        let mut shortcut_active = false;
        let mut smeared_active = false;

        for jc in state.round.jokers.iter() {
            if jc.joker == ortalib::Joker::FourFingers {
                four_fingers_active = true;
            }
            if jc.joker == ortalib::Joker::Shortcut {
                shortcut_active = true;
            }
            if jc.joker == ortalib::Joker::SmearedJoker {
                smeared_active = true;
            }
            if jc.joker == ortalib::Joker::Pareidolia {
                pareidolia_active = true;
            }
            if jc.joker == ortalib::Joker::Splash {
                splash_active = true;
            }
            // Other effect jokers will be handled here as well
        }

        let best_poker_hand = detect_best_hand(
            &state.round.cards_played,
            four_fingers_active,
            shortcut_active,
            smeared_active,
        );
        Self {
            state,
            joker_registry,
            best_poker_hand,
            pareidolia_active,
            splash_active,
        }
    }

    pub fn score(mut self) -> (Chips, Mult) {
        let (base_chips, base_mult) = self.best_poker_hand.0.hand_value();
        self.state.chips += base_chips;
        self.state.mult *= base_mult;

        // Step 2: Score each card
        let scoring_cards = if self.splash_active {
            self.state.round.cards_played.clone()
        } else {
            self.best_poker_hand.1.clone()
        };
        for pc_inner in scoring_cards.into_iter() {
            let mut pc = PlayedCard::new(pc_inner);
            self.score_played_card(&mut pc);
        }

        // Step 3: Held in hand abilities
        let held_wrapped: Vec<PlayedCard> = self
            .state
            .round
            .cards_held_in_hand
            .iter()
            .copied()
            .map(PlayedCard::new)
            .collect();
        for held in held_wrapped.iter() {
            if held.is_steel() {
                self.state.mult *= 1.5;
            }
            let jokers_snapshot = self.state.round.jokers.clone();
            for jc in jokers_snapshot.iter() {
                if let Some(effect) = self.joker_registry.get(&jc.joker) {
                    effect.apply_on_held(&mut self.state, held, jc);
                }
            }
        }

        // Step 4: Joker Editions and "independent" Jokers activate.
        let jokers_snapshot = self.state.round.jokers.clone();
        for jc in jokers_snapshot.iter() {
            crate::scorer::apply_joker_edition(&mut self.state, jc);
            if let Some(effect) = self.joker_registry.get(&jc.joker) {
                effect.apply_independent(&mut self.state, jc, &self.best_poker_hand);
            }
        }

        (self.state.chips, self.state.mult)
    }

    fn score_played_card(&mut self, pc: &mut PlayedCard) {
        self.state.chips += pc.base_chips();
        match pc.inner.enhancement {
            Some(Enhancement::Bonus) => self.state.chips += 30.0,
            Some(Enhancement::Mult) => self.state.mult += 4.0,
            Some(Enhancement::Glass) => self.state.mult *= 2.0,
            _ => {}
        }
        match pc.inner.edition {
            Some(Edition::Foil) => self.state.chips += 50.0,
            Some(Edition::Holographic) => self.state.mult += 10.0,
            Some(Edition::Polychrome) => self.state.mult *= 1.5,
            _ => {}
        }

        // Pareidolia check: all cards are considered face cards
        let is_face = pc.inner.rank.is_face() || self.pareidolia_active;

        let jokers_snapshot = self.state.round.jokers.clone();
        for jc in jokers_snapshot.iter() {
            if let Some(effect) = self.joker_registry.get_mut(&jc.joker) {
                effect.apply_on_scored(&mut self.state, pc, jc);
            }
            if jc.joker == ortalib::Joker::ScaryFace && is_face {
                self.state.chips += 30.0;
            }
            if jc.joker == ortalib::Joker::SmileyFace && is_face {
                self.state.mult += 5.0;
            }
        }

        // Retrigger loop for 'On Scored' abilities
        for _ in 0..pc.retrigger_count {
            let retrigger_jokers_snapshot = self.state.round.jokers.clone();
            for rjc in retrigger_jokers_snapshot.iter() {
                if let Some(effect) = self.joker_registry.get_mut(&rjc.joker) {
                    // Retriggers cannot trigger another retrigger
                    let original_retrigger_count = pc.retrigger_count;
                    pc.retrigger_count = 0;
                    effect.apply_on_scored(&mut self.state, pc, rjc);
                    pc.retrigger_count = original_retrigger_count;
                }
            }
        }
        self.state.mult *= pc.card_mult;
    }
}

pub fn detect_best_hand(
    cards: &[Card],
    four_fingers_active: bool,
    shortcut_active: bool,
    smeared_active: bool,
) -> (PokerHand, Vec<Card>) {
    let rank_groups = group_by_rank(cards);

    let all_same_suit = is_flush_with_smeared_and_flags(cards, four_fingers_active, smeared_active);
    let find_group_by_size = |size: usize| -> Option<Vec<Card>> {
        rank_groups.values().find(|v| v.len() == size).cloned()
    };

    if cards.len() == 5 {
        if rank_groups.len() == 1 && all_same_suit {
            return (PokerHand::FlushFive, cards.to_vec());
        }

        let has_3 = rank_groups.values().any(|v| v.len() == 3);
        let has_2 = rank_groups.values().any(|v| v.len() == 2);
        if has_3 && has_2 && all_same_suit {
            return (PokerHand::FlushHouse, cards.to_vec());
        }

        if let Some(v) = find_group_by_size(5) {
            return (PokerHand::FiveOfAKind, v);
        }

        if is_straight_with_flags(cards, four_fingers_active, shortcut_active) && all_same_suit {
            return (PokerHand::StraightFlush, cards.to_vec());
        }

        if let Some(v) = find_group_by_size(4) {
            return (PokerHand::FourOfAKind, v);
        }

        if has_3 && has_2 {
            return (PokerHand::FullHouse, cards.to_vec());
        }

        if all_same_suit {
            return (PokerHand::Flush, cards.to_vec());
        }

        if is_straight_with_flags(cards, four_fingers_active, shortcut_active) {
            return (PokerHand::Straight, cards.to_vec());
        }
    }

    if let Some(v) = find_group_by_size(3) {
        return (PokerHand::ThreeOfAKind, v);
    }

    let mut pairs: Vec<(&Rank, &Vec<Card>)> =
        rank_groups.iter().filter(|(_, v)| v.len() == 2).collect();
    if pairs.len() >= 2 {
        pairs.sort_by(|a, b| b.0.cmp(a.0));
        let mut cards_out = Vec::new();
        cards_out.extend(pairs[0].1.iter().copied());
        cards_out.extend(pairs[1].1.iter().copied());
        return (PokerHand::TwoPair, cards_out);
    }

    if let Some(v) = find_group_by_size(2) {
        return (PokerHand::Pair, v);
    }

    if let Some(&best) = cards.iter().max() {
        return (PokerHand::HighCard, vec![best]);
    }

    (PokerHand::HighCard, Vec::new())
}

pub fn group_by_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut m = HashMap::new();
    for &c in cards {
        m.entry(c.rank).or_insert_with(Vec::new).push(c);
    }
    m
}

pub fn is_straight_with_flags(
    cards: &[Card],
    four_fingers_active: bool,
    shortcut_active: bool,
) -> bool {
    let min_len = if four_fingers_active { 4 } else { 5 };
    if cards.len() < min_len {
        return false;
    }

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();
    ranks.dedup();
    if ranks.len() < min_len {
        return false;
    }

    fn rank_to_val(r: Rank) -> u8 {
        match r {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    let mut vals: Vec<u8> = ranks.iter().map(|r| rank_to_val(*r)).collect();
    vals.sort();

    // Handle Ace low straight (A, 2, 3, 4, 5)
    let mut ace_low_vals = vals.clone();
    if let Some(pos) = ace_low_vals.iter().position(|&v| v == 14) {
        ace_low_vals[pos] = 1;
        ace_low_vals.sort();
    }

    let check_straight = |current_vals: &[u8]| {
        for i in 0..=current_vals.len() - min_len {
            let sub_slice = &current_vals[i..i + min_len];
            let is_sequential = (0..min_len - 1).all(|j| {
                if shortcut_active {
                    sub_slice[j + 1] - sub_slice[j] <= 2 && sub_slice[j + 1] - sub_slice[j] >= 1
                } else {
                    sub_slice[j + 1] - sub_slice[j] == 1
                }
            });
            if is_sequential {
                return true;
            }
        }
        false
    };

    check_straight(&vals) || check_straight(&ace_low_vals)
}

pub fn is_flush_with_smeared_and_flags(
    cards: &[Card],
    four_fingers_active: bool,
    smeared_active: bool,
) -> bool {
    let min_len = if four_fingers_active || smeared_active {
        4
    } else {
        5
    };
    if cards.len() < min_len {
        return false;
    }

    for &s in &[Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades] {
        if cards
            .iter()
            .filter(|c| {
                let is_smeared_wild =
                    smeared_active && (c.suit == Suit::Hearts || c.suit == Suit::Diamonds);
                c.suit == s || c.enhancement == Some(Enhancement::Wild) || is_smeared_wild
            })
            .count()
            >= min_len
        {
            return true;
        }
    }
    false
}

pub fn apply_joker_edition(state: &mut ScoringState, card: &JokerCard) {
    match card.edition {
        Some(Edition::Foil) => state.chips += 50.0,
        Some(Edition::Holographic) => state.mult += 10.0,
        Some(Edition::Polychrome) => state.mult *= 1.5,
        None => {}
    }
}

pub fn score(round: Round) -> (Chips, Mult) {
    let engine = ScoringEngine::new(round);
    engine.score()
}
