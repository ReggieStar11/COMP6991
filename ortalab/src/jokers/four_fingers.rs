use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct FourFingers;
impl super::JokerEffect for FourFingers {
    fn apply_independent(
        &self,
        _state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
        
    }
}
