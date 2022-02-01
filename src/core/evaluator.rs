use crate::core::card::Card;
use crate::core::evaluator::private_evaluator::Eval;
use crate::core::hand::Hand;
use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum HandValue {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    Trips = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    Quad = 7,
    StraightFlush = 8,
    RoyalFlush = 9
}

const HAND_VALUES: [HandValue; 10] = [
    HandValue::HighCard,
    HandValue::Pair,
    HandValue::TwoPair,
    HandValue::Trips,
    HandValue::Straight,
    HandValue::Flush,
    HandValue::FullHouse,
    HandValue::Quad,
    HandValue::StraightFlush,
    HandValue::RoyalFlush,
];

impl HandValue {
    pub const fn hand_values() -> [Self; 10] {
        HAND_VALUES
    }
}

impl fmt::Display for HandValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandValue::HighCard => write!(f, "HighCard"),
            HandValue::Pair => write!(f, "Pair"),
            HandValue::TwoPair => write!(f, "TwoPair"),
            HandValue::Trips => write!(f, "Trips"),
            HandValue::Straight => write!(f, "Straight"),
            HandValue::Flush => write!(f, "Flush"),
            HandValue::FullHouse => write!(f, "FullHouse"),
            HandValue::Quad => write!(f, "Quad"),
            HandValue::StraightFlush => write!(f, "StraightFlush"),
            HandValue::RoyalFlush => write!(f, "RoyalFlush"),
        }
    }
}
pub struct Evaluator {
}

mod private_evaluator {
    use crate::core::card::Card;
    pub trait Eval {
        fn sort_value(&mut self, hand: & mut Vec<Card>);
        // fn sort_suit(&mut self, hand: & mut Vec<Card>);
        fn is_pair(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_two_pair(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_trips(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_straight(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_flush(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_full_house(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_quad(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_straight_flush(& self, sorted_hand: &Vec<Card>) -> bool;
        fn is_royal_flush(& self, sorted_hand: &Vec<Card>) -> bool;
        fn are2cards_equal(c1: Card, c2: Card) -> bool;
        fn are3cards_equal(c1: Card, c2: Card, c3: Card) -> bool;
        fn are4cards_equal(c1: Card, c2: Card, c3: Card, c4: Card) -> bool;
    }
}

impl private_evaluator::Eval for Evaluator {

    fn sort_value(&mut self, hand: &mut Vec<Card>) {
        hand.sort();
    }

    fn is_pair(& self, sorted_hand: &Vec<Card>) -> bool {
        let first_two = Self::are2cards_equal(sorted_hand[0], sorted_hand[1]);
        let second_two = Self::are2cards_equal(sorted_hand[1], sorted_hand[2]);
        let third_two =	Self::are2cards_equal(sorted_hand[2], sorted_hand[3]);
        let fourth_two = Self::are2cards_equal(sorted_hand[3], sorted_hand[4]);
        if first_two {
            return true
        } else if second_two {
            return true
        } else if third_two {
            return true
        }else if fourth_two {
            return true
        }
        return false
    }

    fn is_two_pair(& self, sorted_hand: &Vec<Card>) -> bool {
        let first_four = Self::are2cards_equal(sorted_hand[0], sorted_hand[1]) && Self::are2cards_equal(sorted_hand[2], sorted_hand[3]);
        let first_last = Self::are2cards_equal(sorted_hand[0], sorted_hand[1]) && Self::are2cards_equal(sorted_hand[3], sorted_hand[4]);
        let last_four =	Self::are2cards_equal(sorted_hand[1], sorted_hand[2]) && Self::are2cards_equal(sorted_hand[3], sorted_hand[4]);
        if first_four {
            return true
        } else if first_last {
            return true
        } else if last_four {
            return true
        }
        return false
    }

    fn is_trips(& self, sorted_hand: &Vec<Card>) -> bool {
        let first_three = Self::are3cards_equal(sorted_hand[0], sorted_hand[1], sorted_hand[2]);
        let middle_three = Self::are3cards_equal(sorted_hand[1], sorted_hand[2], sorted_hand[3]);
        let last_three =	Self::are3cards_equal(sorted_hand[2], sorted_hand[3], sorted_hand[4]);
        if first_three || middle_three || last_three {
            return true
        } else {
            return false
        }
    }

    fn is_straight(& self, sorted_hand: &Vec<Card>) -> bool {
        return (sorted_hand[4].value as u8 - sorted_hand[3].value as u8 == 1) &&
            (sorted_hand[3].value as u8 - sorted_hand[2].value as u8 == 1) &&
            (sorted_hand[2].value as u8 - sorted_hand[1].value as u8 == 1) &&
            (sorted_hand[1].value as u8 - sorted_hand[0].value as u8 == 1)
    }

    fn is_flush(& self, sorted_hand: &Vec<Card>) -> bool {
        return (sorted_hand[0].suit == sorted_hand[1].suit) &&
            (sorted_hand[0].suit == sorted_hand[2].suit) &&
            (sorted_hand[0].suit == sorted_hand[3].suit) &&
            (sorted_hand[0].suit == sorted_hand[4].suit)
    }

    fn is_full_house(& self, sorted_hand: &Vec<Card>) -> bool {
        return (Self::are2cards_equal(sorted_hand[0], sorted_hand[1]) &&
            Self::are3cards_equal(sorted_hand[2], sorted_hand[3], sorted_hand[4])) ||
            (Self::are2cards_equal(sorted_hand[3], sorted_hand[4]) &&
                Self::are3cards_equal(sorted_hand[0], sorted_hand[1], sorted_hand[2]))
    }

    fn is_quad(& self, sorted_hand: &Vec<Card>) -> bool {
        let first_four = Self::are4cards_equal(sorted_hand[0], sorted_hand[1], sorted_hand[2], sorted_hand[3]);
        let last_four =	Self::are4cards_equal(sorted_hand[1], sorted_hand[2], sorted_hand[3], sorted_hand[4]);
        if first_four || last_four {
            return true
        } else {
            return false
        }
    }

    fn is_straight_flush(& self, sorted_hand: &Vec<Card>) -> bool {
        return self.is_straight(sorted_hand) && self.is_flush(sorted_hand);
    }

    fn is_royal_flush(& self, sorted_hand: &Vec<Card>) -> bool {
        return self.is_straight_flush(sorted_hand) && sorted_hand[0].value as u32 == 10;
    }

    fn are2cards_equal(c1: Card, c2: Card) -> bool {
        return c1.value == c2.value
    }
    fn are3cards_equal(c1: Card, c2: Card, c3: Card) -> bool {
        return c1.value == c2.value && c2.value == c3.value
    }

    fn are4cards_equal(c1: Card, c2: Card, c3: Card, c4: Card) -> bool {
        return c1.value == c2.value && c2.value == c3.value && c3.value == c4.value
    }
}

pub trait DoEvaluate: private_evaluator::Eval {
    fn do_evaluate(&mut self, h: &mut Hand) -> HandValue;
}

impl DoEvaluate for Evaluator {
    fn do_evaluate(&mut self, h: &mut Hand) -> HandValue {
        self.sort_value(&mut h.cards);
        // println!("Evaluating: {:?}", h);
        if self.is_royal_flush(&h.cards) {
            return HandValue::RoyalFlush
        } else if self.is_straight_flush(&h.cards) {
            return HandValue::StraightFlush
        } else if self.is_quad(&h.cards) {
            return HandValue::Quad
        } else if self.is_full_house(&h.cards) {
            return HandValue::FullHouse
        } else if self.is_flush(&h.cards) {
            return HandValue::Flush
        } else if self.is_straight(&h.cards) {
            return HandValue::Straight
        } else if self.is_trips(&h.cards) {
            return HandValue::Trips
        } else if self.is_two_pair(&h.cards) {
            return HandValue::TwoPair
        } else if self.is_pair(&h.cards) {
            return HandValue::Pair
        } else {
            HandValue::HighCard
        }
    }
}
