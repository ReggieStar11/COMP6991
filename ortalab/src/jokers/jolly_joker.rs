use crate::scorer::ScoringState;

/// Jolly Joker: +8 Mult if cards_played contains a Pair (independent)
pub struct JollyJoker;
impl super::JokerEffect for JollyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_pair(&state.round.cards_played) {
            state.mult += 8.0;
            state.explain.push("Jolly Joker: +8 Mult (pair present)".to_string());
        }
    }
}