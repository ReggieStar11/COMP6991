use crate::scorer::ScoringState;

/// Devious Joker: +100 Chips if cards_played contains a Straight (independent)
pub struct DeviousJoker;
impl super::JokerEffect for DeviousJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_straight(&state.round.cards_played) {
            state.chips += 100.0;
            state.explain.push("Devious Joker: +100 Chips (straight present)".to_string());
        }
    }
}