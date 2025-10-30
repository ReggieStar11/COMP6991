use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Blueprint;
impl super::JokerEffect for Blueprint {
    fn apply_independent(&self, _state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        // Blueprint's effect involves retriggering other jokers, which requires a more complex mechanism.
        // This will be implemented after a general retriggering mechanism is in place.
    }
}
