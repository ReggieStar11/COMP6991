use crate::scorer::ScoringState;
use ortalib::JokerCard;

/// JokerEffect trait: implement this in each joker file.
pub trait JokerEffect {
    fn apply_independent(
        &self,
        _state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
    }
    fn apply_on_scored(
        &mut self,
        _state: &mut ScoringState,
        _played_card: &mut crate::cards::PlayedCard,
        _joker_card: &JokerCard,
    ) {
    }
    fn apply_on_held(
        &self,
        _state: &mut ScoringState,
        _held_card: &crate::cards::PlayedCard,
        _joker_card: &JokerCard,
    ) {
    }
}

/// Simple Joker: always +4 Mult (independent) plus edition bonuses if present.
pub struct Joker;
impl super::JokerEffect for Joker {
    fn apply_independent(
        &self,
        state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
        state.mult += 4.0;
    }
}
