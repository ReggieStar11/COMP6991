use crate::scorer::ScoringState;

/// Crafty Joker: +80 Chips if cards_played contains a Flush (independent)
pub struct CraftyJoker;
impl super::JokerEffect for CraftyJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_flush(&state.round.cards_played) {
            state.chips += 80.0;
            state.explain.push("Crafty Joker: +80 Chips (flush present)".to_string());
        }
    }
}