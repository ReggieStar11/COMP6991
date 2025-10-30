use crate::scorer::ScoringState;
use ortalib::Edition;

/// JokerEffect trait: implement this in each joker file.
pub trait JokerEffect {
    fn apply_independent(&self, _state: &mut ScoringState) {}
    fn apply_on_scored(&self, _state: &mut ScoringState, _idx: usize) {}
}

/// Simple Joker: always +4 Mult (independent) plus edition bonuses if present.
pub struct Joker;
impl super::JokerEffect for Joker {
    fn apply_independent(&self, state: &mut ScoringState) {
        // +4 mult (base effect)
        state.mult += 4.0;

        // Apply edition bonuses for this joker card
        if let Some(joker_card) = state.round.jokers.iter().find(|jc| jc.joker == ortalib::Joker::Joker) {
            match joker_card.edition {
                Some(Edition::Foil) => state.chips += 50.0,
                Some(Edition::Holographic) => state.mult += 10.0,
                Some(Edition::Polychrome) => state.mult *= 1.5,
                None => {}
            }
        }
    }
}