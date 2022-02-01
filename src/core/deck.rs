
use crate::core::card::{Card, Suit, Value};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for v in &Value::values() {
            for s in &Suit::suits() {
                cards.push(Card {
                    value: *v,
                    suit: *s,
                });
            }
        }
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);

        Self {cards}
    }

    pub fn pop_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

}
