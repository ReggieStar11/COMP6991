use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct Photograph { pub activated: bool }
impl super::JokerEffect for Photograph {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if !self.activated && played_card.inner.rank.is_face() {
            state.mult *= 2.0;
            self.activated = true;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
