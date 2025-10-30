use std::collections::HashMap;
use ortalib::{Card, Chips, Mult, PokerHand, Rank, Round, Suit, Enhancement, Edition};
use crate::jokers;
use crate::cards::PlayedCard;


#[derive(Debug)]
pub struct ScoringState {
    pub round: Round,
    pub chips: Chips,
    pub mult: Mult,
    pub explain: Vec<String>,
}

impl ScoringState {
    pub fn new(round: Round) -> Self {
        Self { round, chips: 0.0, mult: 1.0, explain: Vec::new() }
    }

    pub fn lowest_held_rank_value(&self) -> Option<Chips> {
        self.round
            .cards_held_in_hand
            .iter()
            .map(|c| c.rank.rank_value())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

pub fn group_by_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut m = HashMap::new();
    for &c in cards {
        m.entry(c.rank).or_insert_with(Vec::new).push(c);
    }
    m
}

pub fn group_by_suit(cards: &[Card]) -> HashMap<Suit, Vec<Card>> {
    let mut m = HashMap::new();
    for &c in cards {
        m.entry(c.suit).or_insert_with(Vec::new).push(c);
    }
    m
}

pub fn is_straight(cards: &[Card]) -> bool {
    if cards.len() != 5 { return false; }

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();
    ranks.dedup();
    if ranks.len() != 5 { return false; }

    fn rank_value(r: Rank) -> u8 {
        match r {
            Rank::Two => 2, Rank::Three => 3, Rank::Four => 4, Rank::Five => 5,
            Rank::Six => 6, Rank::Seven => 7, Rank::Eight => 8, Rank::Nine => 9,
            Rank::Ten => 10, Rank::Jack => 11, Rank::Queen => 12, Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    let mut vals: Vec<u8> = ranks.iter().map(|r| rank_value(*r)).collect();
    vals.sort();

    let normal = (0..4).all(|i| vals[i] + 1 == vals[i + 1]);
    let ace_low = vals == vec![2, 3, 4, 5, 14];

    normal || ace_low
}

/// Wild-aware flush detection: a Wild enhancement can act as any suit.
pub fn is_flush_with_wild(cards: &[Card]) -> bool {
    if cards.is_empty() { return false; }

    for &s in &[Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades] {
        if cards.iter().all(|c| c.suit == s || c.enhancement == Some(Enhancement::Wild)) {
            return true;
        }
    }
    false
}

pub fn detect_best_hand(cards: &[Card]) -> (PokerHand, Vec<Card>) {
    let rank_groups = group_by_rank(cards);

    let all_same_suit = is_flush_with_wild(cards);
    let find_group_by_size = |size: usize| -> Option<Vec<Card>> {
        rank_groups.values().find(|v| v.len() == size).map(|v| v.clone())
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

        if is_straight(cards) && all_same_suit {
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

        if is_straight(cards) {
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

pub fn score(round: Round) -> (Chips, Mult) {
    // Build mutable scoring state and joker registry
    let mut state = ScoringState::new(round);
    let registry = jokers::build_registry();

    // Detect best hand (uses raw cards; wild-aware flush detection included)
    let (hand, scoring_cards) = detect_best_hand(&state.round.cards_played);

    // Apply base hand values first
    let (base_chips, base_mult) = hand.hand_value();
    state.chips += base_chips;
    state.mult *= base_mult;

    // Phase 1: independent jokers (apply after base hand values)
    // iterate over a snapshot to avoid borrowing `state` immutably while we mutate it
    let jokers_snapshot = state.round.jokers.clone();
    for jc in jokers_snapshot.iter() {
        if let Some(effect) = registry.get(&jc.joker) {
            effect.apply_independent(&mut state);
        }
    }

    // Wrap scoring cards so we can use PlayedCard helpers
    let scoring_wrapped: Vec<PlayedCard> = scoring_cards.into_iter().map(PlayedCard::new).collect();

    // Per scoring card (left->right)
    for (idx, pc) in scoring_wrapped.iter().enumerate() {
        // base chips using wrapper
        state.chips += pc.base_chips();

        // enhancements applied when the card is scored (access via inner)
        match pc.inner.enhancement {
            Some(Enhancement::Bonus) => state.chips += 30.0,
            Some(Enhancement::Mult) => state.mult += 4.0,
            Some(Enhancement::Glass) => state.mult *= 2.0,
            Some(Enhancement::Wild) => {},
            Some(Enhancement::Steel) => {},
            None => {},
        }

        // editions (Foil/Holographic/Polychrome)
        match pc.inner.edition {
            Some(Edition::Foil) => state.chips += 50.0,
            Some(Edition::Holographic) => state.mult += 10.0,
            Some(Edition::Polychrome) => state.mult *= 1.5,
            None => {},
        }

        // allow jokers to react to this scored card (use snapshot)
        for jc in jokers_snapshot.iter() {
            if let Some(effect) = registry.get(&jc.joker) {
                effect.apply_on_scored(&mut state, idx);
            }
        }
    }

    // Held-in-hand Steel multipliers (left->right)
    let held_wrapped: Vec<PlayedCard> = state.round.cards_held_in_hand.clone().into_iter().map(PlayedCard::new).collect();
    for (held_idx, held) in held_wrapped.iter().enumerate() {
        if held.is_steel() {
            state.mult *= 1.5;
        }

        // allow jokers to react to held cards
        for jc in jokers_snapshot.iter() {
            if let Some(effect) = registry.get(&jc.joker) {
                effect.apply_on_held(&mut state, held_idx);
            }
        }
    }

    (state.chips, state.mult)
}