use crate::scorer::ScoringState;
use ortalib::{JokerCard, Suit, Enhancement};

pub struct FlowerPot;
impl super::JokerEffect for FlowerPot {
    fn apply_independent(&self, state: &mut ScoringState, card: &JokerCard, best_poker_hand: &(ortalib::PokerHand, Vec<ortalib::Card>)) {
        let mut has_hearts = false;
        let mut has_diamonds = false;
        let mut has_clubs = false;
        let mut has_spades = false;

        for played_card in best_poker_hand.1.iter() {
            if played_card.suit == Suit::Hearts || played_card.enhancement == Some(Enhancement::Wild) {
                has_hearts = true;
            }
            if played_card.suit == Suit::Diamonds || played_card.enhancement == Some(Enhancement::Wild) {
                has_diamonds = true;
            }
            if played_card.suit == Suit::Clubs || played_card.enhancement == Some(Enhancement::Wild) {
                has_clubs = true;
            }
            if played_card.suit == Suit::Spades || played_card.enhancement == Some(Enhancement::Wild) {
                has_spades = true;
            }
        }

        if has_hearts && has_diamonds && has_clubs && has_spades {
            state.mult *= 3.0;
        }
        crate::scorer::apply_joker_edition(state, card);
    }
}
