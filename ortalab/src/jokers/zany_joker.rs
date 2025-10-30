use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct ZanyJoker;
impl super::JokerEffect for ZanyJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if crate::jokers::contains_three_of_a_kind(&state.round.cards_played) {
            state.mult += 12.0;
        }
        crate::jokers::apply_joker_edition(state, card);
    }
}