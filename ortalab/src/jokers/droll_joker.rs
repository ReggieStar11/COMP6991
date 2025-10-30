use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct DrollJoker;
impl super::JokerEffect for DrollJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if crate::jokers::contains_flush(&state.round.cards_played) {
            state.mult += 10.0;
        }
        crate::jokers::apply_joker_edition(state, card);
    }
}