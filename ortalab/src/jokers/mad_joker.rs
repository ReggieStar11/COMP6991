use crate::scorer::ScoringState;

/// Mad Joker: +10 Mult if cards_played contains Two Pair (independent)
pub struct MadJoker;
impl super::JokerEffect for MadJoker {
    fn apply_independent(&self, state: &mut ScoringState) {
        if crate::jokers::contains_two_pair(&state.round.cards_played) {
            state.mult += 10.0;
            state.explain.push("Mad Joker: +10 Mult (two-pair present)".to_string());
        }
    }
}