use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct CraftyJoker;
impl super::JokerEffect for CraftyJoker {
    fn apply_independent(&self, state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        if super::contains_flush(&state.round.cards_played, false, false) {
            state.chips += 80.0;
        }
    }
}