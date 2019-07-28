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

    game.tick(75);
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(101);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |",
        "|          |",
    ])

}

#[test]
fn every_100_ms_the_brick_drops_down_one_row() {
    let mut game = Game::new();
    game.tick(50);
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(105);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |",
        "|          |",
    ]);

    game.tick(141);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |",
        "|          |",
    ]);

    game.tick(200);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "| ####     |",
    ]);

}

#[test]
fn drop_multiple_rows_if_enough_time_has_passed() {
    let mut game = Game::new();
    game.tick(50);
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
    ]);

    game.tick(305);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "|          |",
        "| ####     |",
        "|          |",
    ]);
}


#[test]
fn after_brick_reaches_bottom_it_stays_there() {
    let mut game = Game::new();
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
    ]);

    game.tick(500);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
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
        "------------"
    ]);

    game.tick(500 + 900);
    verify_frame(&game, vec![
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
        "| ####     |",
        "|          |",
        "------------"
    ]);

    game.tick(500 + 900 + 500);
    verify_frame(&game, vec![
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
        "| ####     |",
        "------------"
    ]);


}
