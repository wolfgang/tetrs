use super::output_test_helpers::*;

use crate::game::{Game, GameConfig};

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
    let mut game = Game::with_config(&GameConfig { field_height: 6});
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
fn init_time_sets_last_drop_time() {
    let mut game = Game::with_config(&GameConfig { field_height: 6 });
    game.init_time(5000);
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
fn render_returns_number_of_lines_outputted() {
    let game = Game::with_config(&GameConfig { field_height: 6 });
    let (_, number_of_lines) = render_to_cursor(&game);
    assert_eq!(number_of_lines, 7)

}
