use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Mime;
impl super::JokerEffect for Mime {
    fn apply_independent(
        &self,
        _state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
    }
}
