use ortalib::{Card, Chips, Enhancement, Mult};

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

    pub fn base_chips(&self) -> Chips {
        self.inner.rank.rank_value()
    }

    pub fn is_steel(&self) -> bool {
        self.inner.enhancement == Some(Enhancement::Steel)
    }

    pub fn apply_multiplier(&mut self, multiplier: Mult) {
        self.card_mult *= multiplier;
    }
}
