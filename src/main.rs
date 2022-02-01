use std::collections::HashMap;
use poker::core::evaluator::HandValue;
use poker::Game;

fn main() {
    println!("Start Poker 5-card stud!");
    let mut poker_hand_map:HashMap<HandValue, u32> = HashMap::new();
    let cnt = 100000;
    for _n in 0..cnt {
        let mut game = Game::new();
        let results = game.start();

        for r in results {
            *poker_hand_map.entry(r).or_insert(0) += 1;
        }

    }
    show_results(&poker_hand_map, cnt * 10);
}

fn show_results(m:&HashMap<HandValue, u32>, cnt:u32) {
    println!("\nTotal Hands Played:{}", cnt);
    let high_card:f32 = (*m.get(&HandValue::HighCard).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let pair:f32 = (*m.get(&HandValue::Pair).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let two_pair:f32 = (*m.get(&HandValue::TwoPair).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let trips:f32 = (*m.get(&HandValue::Trips).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let straight:f32 = (*m.get(&HandValue::Straight).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let flush:f32 = (*m.get(&HandValue::Flush).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let full_house:f32 = (*m.get(&HandValue::FullHouse).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let quad:f32 = (*m.get(&HandValue::Quad).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let straight_flush:f32 = (*m.get(&HandValue::StraightFlush).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    let royal_flush:f32 = (*m.get(&HandValue::RoyalFlush).unwrap_or(&0) as f32 / cnt as f32) * 100 as f32;
    println!("\nHighCard:{}", high_card);
    println!("Pair:{}", pair);
    println!("TwoPair:{}", two_pair);
    println!("Trips:{}", trips);
    println!("Straight:{}", straight);
    println!("Flush:{}", flush);
    println!("FullHouse:{}", full_house);
    println!("Quad:{}", quad);
    println!("StraightFlush:{}", straight_flush);
    println!("RoyalFlush:{}", royal_flush);
}