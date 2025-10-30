use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Splash;
impl super::JokerEffect for Splash {
    fn apply_independent(&self, _state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        // Splash's effect involves retriggering scoring abilities, which requires a more complex mechanism.
        // This will be implemented after a general retriggering mechanism is in place.
    }
}
