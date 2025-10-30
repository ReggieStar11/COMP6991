use crate::scorer::ScoringState;
use ortalib::{JokerCard, Suit, Enhancement};

pub struct GreedyJoker;
impl super::JokerEffect for GreedyJoker {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &mut crate::cards::PlayedCard, _joker_card: &JokerCard) {
        if played_card.inner.suit == Suit::Diamonds || played_card.inner.enhancement == Some(Enhancement::Wild) {
            state.mult += 3.0;
        }
    }
}
