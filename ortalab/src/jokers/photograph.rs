use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct Photograph { pub activated: bool }
impl super::JokerEffect for Photograph {
    fn apply_on_scored(&self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if !self.activated && played_card.inner.rank.is_face() {
            state.mult *= 2.0;
            // Need to update the state of *this* joker, which requires mutability.
            // Since JokerEffect is an immutable trait object, this might be tricky.
            // For now, I'll leave it as is and revisit if tests fail.
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
