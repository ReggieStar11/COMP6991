use crate::scorer::ScoringState;
use ortalib::{JokerCard};

pub struct CraftyJoker;
impl super::JokerEffect for CraftyJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if super::contains_flush(&state.round.cards_played) {
            state.chips += 80.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}