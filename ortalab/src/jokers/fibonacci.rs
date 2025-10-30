use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct Fibonacci;
impl super::JokerEffect for Fibonacci {
    fn apply_on_scored(&self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        match played_card.inner.rank {
            Rank::Ace | Rank::Two | Rank::Three | Rank::Five | Rank::Eight => state.mult += 8.0,
            _ => {}
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
