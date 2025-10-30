use crate::cards::PlayedCard;
use crate::scorer::ScoringState;
use ortalib::{JokerCard, Rank};

pub struct SockAndBuskin;
impl super::JokerEffect for SockAndBuskin {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &PlayedCard, joker_card: &JokerCard) {
        if played_card.inner.rank.is_face() || played_card.inner.rank == Rank::Ace {
            state.mult *= 2.0;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
