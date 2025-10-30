use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct ZanyJoker;
impl super::JokerEffect for ZanyJoker {
    fn apply_independent(&self, state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        if super::contains_three_of_a_kind(&state.round.cards_played) {
            state.mult += 12.0;
        }
    }
}