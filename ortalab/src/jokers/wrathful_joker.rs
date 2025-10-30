use crate::scorer::ScoringState;
use ortalib::{Enhancement, JokerCard, Suit};

pub struct WrathfulJoker;
impl super::JokerEffect for WrathfulJoker {
    fn apply_on_scored(
        &mut self,
        state: &mut ScoringState,
        played_card: &mut crate::cards::PlayedCard,
        _joker_card: &JokerCard,
    ) {
        if played_card.inner.suit == Suit::Spades
            || played_card.inner.enhancement == Some(Enhancement::Wild)
        {
            state.mult += 3.0;
        }
    }
}
