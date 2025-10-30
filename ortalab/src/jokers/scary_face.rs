use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct ScaryFace;
impl super::JokerEffect for ScaryFace {
    fn apply_on_scored(&mut self, state: &mut ScoringState, played_card: &mut crate::cards::PlayedCard, _joker_card: &JokerCard) {
        if played_card.inner.rank.is_face() {
            state.chips += 30.0;
        }
    }
}
