use std::io::Cursor;
use super::output_test_helpers::*;

use crate::game::Game;

#[test]
fn render_initially_shows_field_with_one_brick() {
    let mut cursor = Cursor::new(Vec::new());
    let game = Game::new();
    game.render(&mut cursor).unwrap();

    assert_output_column(&cursor, vec![
        "| ####     |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "------------"
    ])
}


#[test]
fn after_100_ms_the_brick_drops_down_one_row() {
    let mut cursor = Cursor::new(Vec::new());
    let mut game = Game::new();
    game.tick(101);
    game.render(&mut cursor).unwrap();

    assert_output_column(&cursor, vec![
        "|          |",
        "| ####     |",
        "|          |",
        "|          |",
    ])
}