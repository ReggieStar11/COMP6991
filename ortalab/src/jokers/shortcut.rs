use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Shortcut;
impl super::JokerEffect for Shortcut {
    fn apply_independent(
        &self,
        _state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
    }
}
