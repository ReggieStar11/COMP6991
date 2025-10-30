use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct Baron;
impl super::JokerEffect for Baron {
    fn apply_on_held(&self, state: &mut ScoringState, held_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if held_card.inner.rank == Rank::King {
            state.mult *= 1.5;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
