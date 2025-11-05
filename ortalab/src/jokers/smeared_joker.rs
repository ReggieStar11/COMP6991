use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct SmearedJoker;
impl super::JokerEffect for SmearedJoker {
    fn apply_independent(
        &self,
        _state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
    }
}
