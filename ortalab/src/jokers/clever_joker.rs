use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct CleverJoker;
impl super::JokerEffect for CleverJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if super::contains_two_pair(&state.round.cards_played) {
            state.chips += 80.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}