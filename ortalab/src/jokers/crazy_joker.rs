use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct CrazyJoker;
impl super::JokerEffect for CrazyJoker {
    fn apply_independent(
        &self,
        state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
        if super::contains_straight(&state.round.cards_played, false, false, false) {
            state.mult += 12.0;
        }
    }
}
