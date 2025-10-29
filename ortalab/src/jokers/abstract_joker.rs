use crate::scorer::ScoringState;

/// Abstract Joker: +3 Mult for each Joker card in the round (independent)
pub struct AbstractJoker;
impl super::JokerEffect for AbstractJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        let count = state.round.jokers.len() as f64;
        let bonus = 3.0 * count;
        state.mult += bonus;
        state.explain.push(format!("Abstract Joker: +{} Mult ({} jokers present)", bonus, count));
    }
}