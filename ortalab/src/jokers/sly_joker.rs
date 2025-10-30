use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct SlyJoker;
impl super::JokerEffect for SlyJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if super::contains_pair(&state.round.cards_played) {
            state.chips += 50.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}