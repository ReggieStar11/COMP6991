use crate::scorer::ScoringState;

/// Sly Joker: +50 Chips if cards_played contains a Pair (independent)
pub struct SlyJoker;
impl super::JokerEffect for SlyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_pair(&state.round.cards_played) {
            state.chips += 50.0;
            state.explain.push("Sly Joker: +50 Chips (pair present)".to_string());
        }
    }
}