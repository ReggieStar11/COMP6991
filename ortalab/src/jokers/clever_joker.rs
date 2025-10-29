use crate::scorer::ScoringState;

/// Clever Joker: +80 Chips if cards_played contains Two Pair (independent)
pub struct CleverJoker;
impl super::JokerEffect for CleverJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_two_pair(&state.round.cards_played) {
            state.chips += 80.0;
            state.explain.push("Clever Joker: +80 Chips (two-pair present)".to_string());
        }
    }
}