//! Easy Jokers (Stage 3) module root.
//! Each Joker is implemented in its own file and implements JokerEffect.

use std::collections::HashMap;
use ortalib::{Joker as OrtaJoker};

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
pub mod raised_fist;
pub mod blackboard;
pub mod baron;
pub mod greedy_joker;
pub mod lusty_joker;
pub mod wrathful_joker;
pub mod gluttonous_joker;
pub mod fibonacci;
pub mod scary_face;
pub mod even_steven;
pub mod odd_todd;
pub mod photograph;
pub mod smiley_face;
pub mod flower_pot;
pub mod four_fingers;
pub mod shortcut;
pub mod mime;
pub mod pareidolia;
pub mod splash;
pub mod sock_and_buskin;
pub mod smeared_joker;
pub mod blueprint;

pub use self::joker::JokerEffect;

/// Helper predicates that inspect the played cards using scorer helpers.
fn contains_pair(cards: &[ortalib::Card]) -> bool {
    crate::scorer::group_by_rank(cards).values().any(|v| v.len() >= 2)
}
fn contains_three_of_a_kind(cards: &[ortalib::Card]) -> bool {
    crate::scorer::group_by_rank(cards).values().any(|v| v.len() >= 3)
}
fn contains_two_pair(cards: &[ortalib::Card]) -> bool {
    let groups = crate::scorer::group_by_rank(cards);
    let pairs = groups.values().filter(|v| v.len() == 2).count();
    pairs >= 2
}
fn contains_straight(cards: &[ortalib::Card], four_fingers_active: bool, shortcut_active: bool, _smeared_active: bool) -> bool {
    crate::scorer::is_straight_with_flags(cards, four_fingers_active, shortcut_active)
}
fn contains_flush(cards: &[ortalib::Card], four_fingers_active: bool, _smeared_active: bool) -> bool {
    crate::scorer::is_flush_with_smeared_and_flags(cards, four_fingers_active, _smeared_active)
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
    m.insert(J::RaisedFist, Box::new(raised_fist::RaisedFist {}));
    m.insert(J::Blackboard, Box::new(blackboard::Blackboard {}));
    m.insert(J::Baron, Box::new(baron::Baron {}));
    m.insert(J::GreedyJoker, Box::new(greedy_joker::GreedyJoker {}));
    m.insert(J::LustyJoker, Box::new(lusty_joker::LustyJoker {}));
    m.insert(J::WrathfulJoker, Box::new(wrathful_joker::WrathfulJoker {}));
    m.insert(J::GluttonousJoker, Box::new(gluttonous_joker::GluttonousJoker {}));
    m.insert(J::Fibonacci, Box::new(fibonacci::Fibonacci {}));
    m.insert(J::ScaryFace, Box::new(scary_face::ScaryFace {}));
    m.insert(J::EvenSteven, Box::new(even_steven::EvenSteven {}));
    m.insert(J::OddTodd, Box::new(odd_todd::OddTodd {}));
    m.insert(J::Photograph, Box::new(photograph::Photograph { activated: false }));
    m.insert(J::SmileyFace, Box::new(smiley_face::SmileyFace {}));
    m.insert(J::FlowerPot, Box::new(flower_pot::FlowerPot {}));
    m.insert(J::FourFingers, Box::new(four_fingers::FourFingers {}));
    m.insert(J::Shortcut, Box::new(shortcut::Shortcut {}));
    m.insert(J::Mime, Box::new(mime::Mime {}));
    m.insert(J::Pareidolia, Box::new(pareidolia::Pareidolia {}));
    m.insert(J::Splash, Box::new(splash::Splash {}));
    m.insert(J::SockAndBuskin, Box::new(sock_and_buskin::SockAndBuskin {}));
    m.insert(J::SmearedJoker, Box::new(smeared_joker::SmearedJoker {}));
    m.insert(J::Blueprint, Box::new(blueprint::Blueprint {}));

    m
}