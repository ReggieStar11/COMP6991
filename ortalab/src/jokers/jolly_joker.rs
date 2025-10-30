use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct JollyJoker;
impl super::JokerEffect for JollyJoker {
    fn apply_independent(&self, state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        if super::contains_pair(&state.round.cards_played) {
            state.mult += 8.0;
        }
    }
}