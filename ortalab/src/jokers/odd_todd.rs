use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct OddTodd;
impl super::JokerEffect for OddTodd {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &mut crate::cards::PlayedCard, _joker_card: &JokerCard) {
        match played_card.inner.rank {
            Rank::Ace | Rank::Nine | Rank::Seven | Rank::Five | Rank::Three => state.chips += 31.0,
            _ => {}
        }
    }
}
