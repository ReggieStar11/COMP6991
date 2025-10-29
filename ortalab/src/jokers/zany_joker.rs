use crate::scorer::ScoringState;

/// Zany Joker: +12 Mult if cards_played contains Three of a Kind (independent)
pub struct ZanyJoker;
impl super::JokerEffect for ZanyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_three_of_a_kind(&state.round.cards_played) {
            state.mult += 12.0;
            state.explain.push("Zany Joker: +12 Mult (three-of-a-kind present)".to_string());
        }
    }
}