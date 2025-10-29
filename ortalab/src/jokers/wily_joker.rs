use crate::scorer::ScoringState;

/// Wily Joker: +100 Chips if cards_played contains Three of a Kind (independent)
pub struct WilyJoker;
impl super::JokerEffect for WilyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_three_of_a_kind(&state.round.cards_played) {
            state.chips += 100.0;
            state.explain.push("Wily Joker: +100 Chips (three-of-a-kind present)".to_string());
        }
    }
}