use ortalib::{Card, Chips, Edition, Enhancement, Mult, Rank, Suit};

#[derive(Clone, Copy, Debug)]
pub struct PlayedCard {
    pub inner: Card,
    pub card_mult: Mult,
    pub retrigger_count: u8,
}

impl PlayedCard {
    pub fn new(c: Card) -> Self {
        Self {
            inner: c,
            card_mult: 1.0,
            retrigger_count: 0,
        }
    }

    pub fn _rank(&self) -> Rank {
        self.inner.rank
    }
    pub fn _suit(&self) -> Suit {
        self.inner.suit
    }
    pub fn base_chips(&self) -> Chips {
        self.inner.rank.rank_value()
    }

    pub fn _is_face(&self) -> bool {
        self.inner.rank.is_face()
    }
    pub fn _is_wild(&self) -> bool {
        self.inner.enhancement == Some(Enhancement::Wild)
    }
    pub fn is_steel(&self) -> bool {
        self.inner.enhancement == Some(Enhancement::Steel)
    }

    pub fn _edition_is_polychrome(&self) -> bool {
        self.inner.edition == Some(Edition::Polychrome)
    }

    pub fn apply_multiplier(&mut self, multiplier: Mult) {
        self.card_mult *= multiplier;
    }
}
