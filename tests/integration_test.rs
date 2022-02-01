use poker;

mod common;
use poker::Game;

#[test]
fn it_has_5_cards_each_hand() {
    common::setup();
    let mut game = Game::new();

    game.start();
    assert_eq!(5, game.hand1.len());
}