use crate::scorer::ScoringState;
use ortalib::JokerCard;

/// JokerEffect trait: implement this in each joker file.
pub trait JokerEffect {
    fn apply_independent(&self, _state: &mut ScoringState, _card: &JokerCard) {}
}

/// Simple Joker: always +4 Mult (independent) plus edition bonuses if present.
pub struct Joker;
impl super::JokerEffect for Joker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard) {
        state.mult += 4.0;
        crate::scorer::apply_joker_edition(state, card);
    }
}
