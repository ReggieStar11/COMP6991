use crate::scorer::ScoringState;
use ortalib::{Enhancement, JokerCard, Suit};

pub struct Blackboard;
impl super::JokerEffect for Blackboard {
    fn apply_independent(
        &self,
        state: &mut ScoringState,
        _card: &JokerCard,
        _best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>),
    ) {
        let all_black_suits = state.round.cards_held_in_hand.iter().all(|c| {
            c.suit == Suit::Spades
                || c.suit == Suit::Clubs
                || c.enhancement == Some(Enhancement::Wild)
        });

        if all_black_suits {
            state.mult *= 3.0;
        }
    }
}
