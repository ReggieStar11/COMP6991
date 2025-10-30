use crate::scorer::ScoringState;
use ortalib::{JokerCard, Suit, Enhancement};

pub struct WrathfulJoker;
impl super::JokerEffect for WrathfulJoker {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if played_card.inner.suit == Suit::Spades || played_card.inner.enhancement == Some(Enhancement::Wild) {
            state.mult += 3.0;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
