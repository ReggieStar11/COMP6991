use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Blueprint;
impl super::JokerEffect for Blueprint {
    fn apply_independent(&self, _state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        // Unable to implement the Blueprint joker effect
    }
}
