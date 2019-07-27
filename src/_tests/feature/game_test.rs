use std::io::Cursor;
use super::output_test_helpers::*;

use crate::game::Game;

#[test]
fn render_initially_shows_field_with_one_brick() {
    let mut cursor = Cursor::new(Vec::new());

    let game = Game::new();

    game.render(&mut cursor).unwrap();

    assert_output(&cursor, vec![
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
        "|          |",
        "------------"
    ])
}