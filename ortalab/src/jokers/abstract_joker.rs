use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct AbstractJoker;
impl super::JokerEffect for AbstractJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        let num_jokers = state.round.jokers.len() as f64;
        state.mult += 3.0 * num_jokers;
        crate::scorer::apply_joker_edition(state, card);
    }
}
