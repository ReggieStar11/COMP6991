use crate::scorer::ScoringState;

/// Crazy Joker: +12 Mult if cards_played contains a Straight (independent)
pub struct CrazyJoker;
impl super::JokerEffect for CrazyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_straight(&state.round.cards_played) {
            state.mult += 12.0;
            state.explain.push("Crazy Joker: +12 Mult (straight present)".to_string());
        }
    }
}