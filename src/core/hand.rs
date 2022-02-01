
use crate::core::card::Card;
// use std::slice::Iter;
use std::fmt;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(5),
        }
    }

    pub fn push(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    // pub fn iter(&self) -> Iter<Card> {
    //     self.cards.iter()
    // }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Card, Suit, Value};

    #[test]
    fn test_add_card() {
        let mut h = Hand::new();
        let c = Card { value: Value::Three, suit: Suit::Spade, };
        h.push(c);

        assert_eq!(1, h.len());
    }

}

