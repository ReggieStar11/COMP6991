use crate::scorer::ScoringState;

/// JokerEffect trait: implement this in each joker file.
pub trait JokerEffect {
    fn apply_independent(&self, _state: &mut ScoringState) {}
    fn apply_on_scored(&self, _state: &mut ScoringState, _idx: usize) {}
    fn apply_on_held(&self, _state: &mut ScoringState, _idx: usize) {}
}

/// Simple Joker: always +4 Mult (independent)
pub struct Joker;
impl super::JokerEffect for Joker {
    fn apply_independent(&self, state: &mut ScoringState) {
        state.mult += 4.0;
        state.explain.push("Joker: +4 Mult (independent)".to_string());
    }
}