use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct MadJoker;
impl super::JokerEffect for MadJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if crate::jokers::contains_two_pair(&state.round.cards_played) {
            state.mult += 10.0;
        }
        crate::jokers::apply_joker_edition(state, card);
    }
}