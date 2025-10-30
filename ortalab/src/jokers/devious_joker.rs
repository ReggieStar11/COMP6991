use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct DeviousJoker;
impl super::JokerEffect for DeviousJoker {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        if super::contains_straight(&state.round.cards_played, false, false, false) {
            state.chips += 100.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}