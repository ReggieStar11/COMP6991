use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct DrollJoker;
impl super::JokerEffect for DrollJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if super::contains_flush(&state.round.cards_played) {
            state.mult += 10.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}