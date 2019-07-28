use super::output_test_helpers::*;

use crate::game::Game;

#[test]
fn render_initially_shows_field_with_one_brick() {
    let game = Game::default();
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
fn every_100_ms_the_brick_drops_down_one_row() {
    let mut game = Game::default();
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
    let mut game = Game::default();
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
    let mut game = Game::init().with_field_height(6).build();
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "------------"
    ]);

    game.tick(300);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "|          |",
        "| ####     |",
        "|          |",
        "|          |",
        "------------"
    ]);

    game.tick(300 + 700);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "| ####     |",
        "------------"
    ]);

    game.tick(300 + 700 + 1000);
    verify_frame(&game, vec![
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "| ####     |",
        "------------"
    ]);
}

#[test]
fn can_use_game_builder_to_override_initial_time() {
    let mut game = Game::init()
        .with_field_height(6)
        .with_now_millis(5000)
        .build();

    game.tick(5000 + 100);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |",
        "|          |",
        "|          |",
        "|          |",
        "|          |",
        "------------"
    ]);
}

#[test]
fn can_use_game_builder_to_override_drop_interval() {
    let mut game = Game::init()
        .with_drop_interval(250)
        .build();

    game.tick(150);
    verify_frame(&game, vec![
        "| ####     |",
        "|          |",
    ]);

    game.tick(250);
    verify_frame(&game, vec![
        "|          |",
        "| ####     |"
    ]);
}


#[test]
fn render_returns_number_of_lines_outputted() {
    let game = Game::init().with_field_height(6).build();
    let (_, number_of_lines) = render_to_cursor(&game);
    assert_eq!(number_of_lines, 7)
}
