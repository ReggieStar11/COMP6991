use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct RaisedFist;
impl super::JokerEffect for RaisedFist {
    fn apply_on_held(
        &self,
        state: &mut ScoringState,
        held_card: &crate::cards::PlayedCard,
        joker_card: &JokerCard,
    ) {
        if let Some(lowest_held) = state.find_lowest_rightmost_held_card()
            && held_card.inner.rank == lowest_held.inner.rank
            && held_card.inner.suit == lowest_held.inner.suit
        {
            state.mult += lowest_held.inner.rank.rank_value() * 2.0;
        }
        crate::scorer::apply_joker_edition(state, joker_card);
    }
}
