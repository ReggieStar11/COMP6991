use std::collections::HashMap;
use crate::scorer::ScoringState;
use ortalib::Joker;

pub trait JokerEffect {
    fn apply_independent(&self, _state: &mut ScoringState) {}
    fn apply_on_scored(&self, _state: &mut ScoringState, _idx: usize) {}
    fn apply_on_held(&self, _state: &mut ScoringState, _idx: usize) {}
}

pub fn build_registry() -> HashMap<Joker, Box<dyn JokerEffect>> {
    HashMap::new()
}