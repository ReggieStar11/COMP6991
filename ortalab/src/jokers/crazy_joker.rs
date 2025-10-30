use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct CrazyJoker;
impl super::JokerEffect for CrazyJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        if super::contains_straight(&state.round.cards_played) {
            state.mult += 12.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}
