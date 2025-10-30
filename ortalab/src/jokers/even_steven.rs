use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct EvenSteven;
impl super::JokerEffect for EvenSteven {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &mut crate::cards::PlayedCard, _joker_card: &JokerCard) {
        match played_card.inner.rank {
            Rank::Ten | Rank::Eight | Rank::Six | Rank::Four | Rank::Two => state.mult += 4.0,
            _ => {}
        }
    }
}
