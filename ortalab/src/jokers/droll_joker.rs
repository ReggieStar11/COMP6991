use crate::scorer::ScoringState;

/// Droll Joker: +10 Mult if cards_played contains a Flush (independent)
pub struct DrollJoker;
impl super::JokerEffect for DrollJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_flush(&state.round.cards_played) {
            state.mult += 10.0;
            state.explain.push("Droll Joker: +10 Mult (flush present)".to_string());
        }
    }
}