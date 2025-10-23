use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

use clap::Parser;
use std::collections::HashMap;
use ortalib::{Card, Chips, Mult, PokerHand, Rank, Round};

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn group_by_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut m = HashMap::new();
    for &c in cards {
        m.entry(c.rank).or_insert_with(Vec::new).push(c);
    }
    m
}

fn group_by_suit(cards: &[Card]) -> HashMap<ortalib::Suit, Vec<Card>> {
    let mut m = HashMap::new();
    for &c in cards {
        m.entry(c.suit).or_insert_with(Vec::new).push(c);
    }
    m
}

fn is_straight(cards: &[Card]) -> bool {
    if cards.len() != 5 {
        return false;
    }

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();
    ranks.dedup();
    if ranks.len() != 5 {
        return false;
    }

    // map Rank to integer values for ordinal comparison
    fn rank_value(r: Rank) -> u8 {
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

    let mut vals: Vec<u8> = ranks.iter().map(|r| rank_value(*r)).collect();
    vals.sort();

    // normal consecutive check using integer values
    let normal = (0..4).all(|i| vals[i] + 1 == vals[i + 1]);

    // special case A-2-3-4-5 (Ace low)
    let ace_low = vals == vec![2, 3, 4, 5, 14];

    normal || ace_low
}

fn detect_best_hand(cards: &[Card]) -> (PokerHand, Vec<Card>) {

    let rank_groups = group_by_rank(cards);
    let suit_groups = group_by_suit(cards);

    // helper closures
    let all_same_suit = suit_groups.len() == 1 && cards.len() >= 1;
    let find_group_by_size = |size: usize| -> Option<Vec<Card>> {
        rank_groups
            .values()
            .find(|v| v.len() == size)
            .map(|v| v.clone())
    };

    // 5-card specific checks first (only possible when 5 played)
    if cards.len() == 5 {
        // FlushFive: all same rank and same suit
        if rank_groups.len() == 1 && all_same_suit {
            return (PokerHand::FlushFive, cards.to_vec());
        }

        // FlushHouse: 3+2 and all same suit
        let has_3 = rank_groups.values().any(|v| v.len() == 3);
        let has_2 = rank_groups.values().any(|v| v.len() == 2);
        if has_3 && has_2 && all_same_suit {
            return (PokerHand::FlushHouse, cards.to_vec());
        }

        // FiveOfAKind: any rank group of size 5 (not all same suit)
        if let Some(v) = find_group_by_size(5) {
            return (PokerHand::FiveOfAKind, v);
        }

        // StraightFlush
        if is_straight(cards) && all_same_suit {
            return (PokerHand::StraightFlush, cards.to_vec());
        }

        // FourOfAKind
        if let Some(v) = find_group_by_size(4) {
            return (PokerHand::FourOfAKind, v);
        }

        // FullHouse (3 + 2, but not flush house since handled)
        if has_3 && has_2 {
            return (PokerHand::FullHouse, cards.to_vec());
        }

        // Flush
        if all_same_suit {
            return (PokerHand::Flush, cards.to_vec());
        }

        // Straight
        if is_straight(cards) {
            return (PokerHand::Straight, cards.to_vec());
        }
    }

    // For hands that may occur with fewer than 5 cards or remaining checks:
    // Three of a kind
    if let Some(v) = find_group_by_size(3) {
        return (PokerHand::ThreeOfAKind, v);
    }

    // Two Pair (find two distinct rank groups of size 2)
    let mut pairs: Vec<(&Rank, &Vec<Card>)> = rank_groups
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .collect();
    if pairs.len() >= 2 {
        // choose the two highest pairs (sort by Rank desc)
        pairs.sort_by(|a, b| b.0.cmp(a.0));
        let mut cards_out = Vec::new();
        cards_out.extend(pairs[0].1.iter().copied());
        cards_out.extend(pairs[1].1.iter().copied());
        return (PokerHand::TwoPair, cards_out);
    }

    // Pair
    if let Some(v) = find_group_by_size(2) {
        return (PokerHand::Pair, v);
    }

    // High Card: single highest card
    if let Some(&best) = cards.iter().max() {
        return (PokerHand::HighCard, vec![best]);
    }

    // fallback (shouldn't happen because round has >=1 card)
    (PokerHand::HighCard, Vec::new())
}


fn score(round: Round) -> (Chips, Mult) {
    let (hand, scoring_cards) = detect_best_hand(&round.cards_played);

    let (base_chips, mult) = hand.hand_value();
    let cards_chips: Chips = scoring_cards.iter().map(|c| c.rank.rank_value()).sum();

    (base_chips + cards_chips, mult)
}
