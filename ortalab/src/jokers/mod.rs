//! Easy Jokers (Stage 3) module root.
//! Each Joker is implemented in its own file and implements JokerEffect.

use std::collections::HashMap;
use ortalib::Joker as OrtaJoker;

pub mod joker;
pub mod jolly_joker;
pub mod zany_joker;
pub mod mad_joker;
pub mod crazy_joker;
pub mod droll_joker;
pub mod sly_joker;
pub mod wily_joker;
pub mod clever_joker;
pub mod devious_joker;
pub mod crafty_joker;
pub mod abstract_joker;

pub use self::joker::JokerEffect;

/// Helper predicates that inspect the played cards using scorer helpers.
fn contains_pair(cards: &[ortalib::Card]) -> bool {
    let groups = crate::scorer::group_by_rank(cards);
    groups.values().any(|v| v.len() >= 2)
}
fn contains_three_of_a_kind(cards: &[ortalib::Card]) -> bool {
    let groups = crate::scorer::group_by_rank(cards);
    groups.values().any(|v| v.len() >= 3)
}
fn contains_two_pair(cards: &[ortalib::Card]) -> bool {
    let groups = crate::scorer::group_by_rank(cards);
    let pairs = groups.values().filter(|v| v.len() == 2).count();
    pairs >= 2
}
fn contains_straight(cards: &[ortalib::Card]) -> bool {
    crate::scorer::is_straight(cards)
}
fn contains_flush(cards: &[ortalib::Card]) -> bool {
    crate::scorer::is_flush_with_wild(cards)
}

/// Build registry mapping ortalib::Joker -> boxed behaviour.
pub fn build_registry() -> HashMap<OrtaJoker, Box<dyn JokerEffect>> {
    use OrtaJoker as J;
    let mut m: HashMap<OrtaJoker, Box<dyn JokerEffect>> = HashMap::new();

    // Register each easy joker (ensure ortalib::Joker has these variants in your version)
    m.insert(J::Joker, Box::new(joker::Joker {}));
    m.insert(J::JollyJoker, Box::new(jolly_joker::JollyJoker {}));
    m.insert(J::ZanyJoker, Box::new(zany_joker::ZanyJoker {}));
    m.insert(J::MadJoker, Box::new(mad_joker::MadJoker {}));
    m.insert(J::CrazyJoker, Box::new(crazy_joker::CrazyJoker {}));
    m.insert(J::DrollJoker, Box::new(droll_joker::DrollJoker {}));
    m.insert(J::SlyJoker, Box::new(sly_joker::SlyJoker {}));
    m.insert(J::WilyJoker, Box::new(wily_joker::WilyJoker {}));
    m.insert(J::CleverJoker, Box::new(clever_joker::CleverJoker {}));
    m.insert(J::DeviousJoker, Box::new(devious_joker::DeviousJoker {}));
    m.insert(J::CraftyJoker, Box::new(crafty_joker::CraftyJoker {}));
    m.insert(J::AbstractJoker, Box::new(abstract_joker::AbstractJoker {}));

    m
}