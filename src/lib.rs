
pub mod core;

use crate::core::{dealer::Dealer, hand::Hand};
use crate::core::evaluator::{DoEvaluate, Evaluator, HandValue};

pub struct Game {
    dealer: Dealer,
    pub hand1: Hand,
    pub hand2: Hand,
}

impl Game {
    pub fn new() -> Self {
        let dealer = Dealer::new();
        let hand1 = Hand::new();
        let hand2 = Hand::new();

        Self {dealer, hand1, hand2}
    }

    pub fn start(&mut self) -> Vec<HandValue> {
        let d = &mut self.dealer;
        let mut results = Vec::with_capacity(10);
        for _n in 0..5 {
            d.deal((&mut self.hand1, &mut self.hand2));
            let mut ev = Evaluator{};
            let poker_hand1 = ev.do_evaluate(&mut self.hand1);
            let poker_hand2 = ev.do_evaluate(&mut self.hand2);
            results.push(poker_hand1);
            results.push(poker_hand2);
            if poker_hand1 > HandValue::Quad {
                println!("Bob: {}{}", poker_hand1, self.hand1);
            }
            if poker_hand2 > HandValue::Quad {
                println!("Larry: {}{}", poker_hand2, self.hand2);
            }
            self.hand1.cards = vec![];
            self.hand2.cards = vec![];
        }
        results
    }
}

