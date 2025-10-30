use crate::scorer::ScoringState;
use ortalib::{JokerCard, Suit, Enhancement};

pub struct LustyJoker;
impl super::JokerEffect for LustyJoker {
    fn apply_on_scored(&self, state: &mut ScoringState, played_card: &crate::cards::PlayedCard, joker_card: &JokerCard) {
        if played_card.inner.suit == Suit::Hearts || played_card.inner.enhancement == Some(Enhancement::Wild) {
            state.mult += 3.0;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
