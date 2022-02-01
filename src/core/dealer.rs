
use crate::core::{deck::Deck, hand::Hand};

pub struct Dealer {
    pub deck: Deck
}

impl Dealer {
    pub fn new() -> Self {
        let deck: Deck = Deck::new();
        Self {deck}
    }

    pub fn deal(&mut self, hands: (&mut Hand, &mut Hand)) {
        let mut i = 0;
        while i < 5 {
            hands.0.push(self.deck.pop_card());
            hands.1.push(self.deck.pop_card());
            i += 1;
        }
    }
}

