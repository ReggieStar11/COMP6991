use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct Fibonacci;
impl super::JokerEffect for Fibonacci {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &mut crate::cards::PlayedCard, _joker_card: &JokerCard) {
        match played_card.inner.rank {
            Rank::Ace | Rank::Two | Rank::Three | Rank::Five | Rank::Eight => state.mult += 8.0,
            _ => {}
        }
    }
}
