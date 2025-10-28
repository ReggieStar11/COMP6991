use ortalib::{Card, Chips, Rank, Suit, Enhancement, Edition};

#[derive(Clone, Copy, Debug)]
pub struct PlayedCard { pub inner: Card }

impl PlayedCard {
    pub fn new(c: Card) -> Self { Self { inner: c } }

    pub fn rank(&self) -> Rank { self.inner.rank }
    pub fn suit(&self) -> Suit { self.inner.suit }
    pub fn base_chips(&self) -> Chips { self.inner.rank.rank_value() }

    pub fn is_wild(&self) -> bool { self.inner.enhancement == Some(Enhancement::Wild) }
    pub fn is_steel(&self) -> bool { self.inner.enhancement == Some(Enhancement::Steel) }

    pub fn edition_is_polychrome(&self) -> bool { self.inner.edition == Some(Edition::Polychrome) }
}