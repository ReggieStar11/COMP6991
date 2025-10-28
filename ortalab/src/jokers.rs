use std::collections::HashMap;
use crate::scorer::ScoringState;
use ortalib::Joker;

/// Minimal trait for joker behaviours (stubs).
pub trait JokerEffect {
    fn apply_independent(&self, _state: &mut ScoringState) {}
    fn apply_on_scored(&self, _state: &mut ScoringState, _idx: usize) {}
    fn apply_on_held(&self, _state: &mut ScoringState, _idx: usize) {}
}

/// Registry stub — add implementations as you implement jokers.
pub fn build_registry() -> HashMap<Joker, Box<dyn JokerEffect>> {
    HashMap::new()
}