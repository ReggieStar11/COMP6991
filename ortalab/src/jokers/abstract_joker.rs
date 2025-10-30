use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct AbstractJoker;
impl super::JokerEffect for AbstractJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        let count = state.round.jokers.len() as f64;
        state.mult += 3.0 * count;
        crate::jokers::apply_joker_edition(state, card);
    }
}