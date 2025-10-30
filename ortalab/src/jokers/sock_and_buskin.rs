use crate::cards::PlayedCard;
use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct SockAndBuskin;
impl super::JokerEffect for SockAndBuskin {
    fn apply_on_scored(
        &mut self,
        _state: &mut ScoringState,
        played_card: &mut PlayedCard,
        _joker_card: &JokerCard,
    ) {
        if played_card.inner.rank.is_face() {
            played_card.apply_multiplier(2.0);
            played_card.retrigger_count += 1; // Signal 1 retrigger for Face cards
        } else if played_card.inner.rank == Rank::Ace {
            played_card.apply_multiplier(2.0);
            played_card.retrigger_count += 2; // Signal 2 retriggers for Aces
        }
    }
}
