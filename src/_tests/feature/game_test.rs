use std::io::Cursor;
use super::output_test_helpers::*;

use crate::game::Game;

#[test]
fn render_initially_shows_field_with_one_brick() {
    let game = Game::new();
    let output = render_game(&game);

    assert_output_column(&output, vec![
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
    let mut game = Game::new();
    game.tick(30);
    let output = render_game(&game);

    assert_output_column(&output, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(30);
    let output = render_game(&game);

    assert_output_column(&output, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(41);
    let output = render_game(&game);

    assert_output_column(&output, vec![
        "|          |",
        "| ####     |",
        "|          |",
    ])

}