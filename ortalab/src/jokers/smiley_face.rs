use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct SmileyFace;
impl super::JokerEffect for SmileyFace {
    fn apply_on_scored(&self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if played_card.inner.rank.is_face() {
            state.mult += 5.0;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
