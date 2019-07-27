use super::output_test_helpers::*;

use crate::game::Game;

#[test]
fn render_initially_shows_field_with_one_brick() {
    let game = Game::new();
    verify_frame(&game, vec![
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
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(30);
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(41);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |",
        "|          |",
    ])

}