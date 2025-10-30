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
        // Mime's effect involves retriggering other jokers, which requires a more complex mechanism.
        // This will be implemented after a general retriggering mechanism is in place.
    }
}
