use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct DeviousJoker;
impl super::JokerEffect for DeviousJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if crate::jokers::contains_straight(&state.round.cards_played) {
            state.chips += 100.0;
        }
        crate::jokers::apply_joker_edition(state, card);
    }
}