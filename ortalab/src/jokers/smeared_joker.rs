use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct SmearedJoker;
impl super::JokerEffect for SmearedJoker {
    fn apply_independent(&self, _state: &mut ScoringState, _card: &JokerCard, _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        // Smeared Joker's effect (making Hearts and Diamonds Wild, and modifying Flush hand size)
        // requires flags to be set in ScoringEngine before hand detection.
        // This will be handled in ScoringEngine::new, similar to FourFingers and Shortcut.
    }
}
