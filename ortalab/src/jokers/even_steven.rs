use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct EvenSteven;
impl super::JokerEffect for EvenSteven {
    fn apply_on_scored(&self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        match played_card.inner.rank {
            Rank::Ten | Rank::Eight | Rank::Six | Rank::Four | Rank::Two => state.mult += 4.0,
            _ => {}
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
