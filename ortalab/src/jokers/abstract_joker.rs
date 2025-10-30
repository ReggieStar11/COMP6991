use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct AbstractJoker;
impl super::JokerEffect for AbstractJoker {
    fn apply_independent(
        &self,
        state: &mut ScoringState,
        card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
        let num_jokers = state.round.jokers.len() as f64;
        state.mult += 3.0 * num_jokers;
        crate::scorer::apply_joker_edition(state, card);
    }
}
