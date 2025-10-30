use crate::scorer::ScoringState;
use ortalib::JokerCard;

pub struct Pareidolia;
impl super::JokerEffect for Pareidolia {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard, best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        let all_face_cards = best_poker_hand.1.iter().all(|c| c.rank.is_face());

        if all_face_cards {
            state.mult *= 2.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}
