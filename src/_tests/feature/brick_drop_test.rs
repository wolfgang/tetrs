use crate::_tests::helpers::to_string_renderer::*;
use crate::game::Game;
use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, o_block};

#[test]
fn render_initially_shows_field_with_one_brick() {
    let mut game = TestableGame::default();

    game.render();
    game.assert_frame_exact(vec![
        ".####.....",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
    ])
}

#[test]
fn every_100_ms_since_last_drop_the_brick_drops_down_one_row() {
    let mut game = TestableGame::default();

    game.verify_frame_after(50, vec![
        ".####.....",
        "..........",
        "..........",
    ]);

    game.verify_frame_after(105, vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.verify_frame_after(160, vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.verify_frame_after(205, vec![
        "..........",
        "..........",
        ".####....."
    ]);
}


#[test]
fn after_brick_reaches_bottom_it_stays_there() {
    let mut game = TestableGame::init().with_field_height(3).build();

    game.verify_exact_frame_after(100, vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.verify_exact_frame_after(200, vec![
        "..........",
        "..........",
        ".####....."
    ]);

    game.verify_exact_frame_after(300, vec![
        ".####.....",
        "..........",
        ".####....."
    ]);
}

#[test]
fn can_use_game_builder_to_override_initial_time() {
    let mut game = Game::init()
        .with_now_millis(5000)
        .build();
    let mut renderer = ToStringRenderer::default();

    game.tick(5000 + 100);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####.....",
        "..........",
        "..........",
        "..........",
        ".........."
    ]);
}


#[test]
fn can_use_game_builder_to_override_drop_interval() {
    let mut game = Game::init()
        .with_drop_interval(250)
        .build();
    let mut renderer = ToStringRenderer::default();

    game.tick(150);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ".####.....",
        "..........",
    ]);

    game.tick(250);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####....."
    ]);
}


#[test]
fn reset_rotation_for_new_brick() {
    let mut game = TestableGame::init()
        .with_field_height(5)
        .build();

    game.verify_frame_after(0, vec![
        ".####.....",
        "..........",
    ]);

    game.tick(100);
    game.is_rotating();
    game.verify_frame_after(150, vec![
        "..........",
        "...#......",
        "...#......",
        "...#......",
        "...#......",
    ]);

    game.stop_rotating();

    game.verify_frame_after(200, vec![
        ".####.....",
        "...#......",
        "...#......",
        "...#......",
        "...#......",
    ]);
}

#[test]
fn reduce_drop_interval_if_input_calls_for_it() {
    let mut game = TestableGame::init()
        .with_drop_interval(100)
        .with_brick_sequence(vec![i_block(), o_block()])
        .build();

    game.verify_frame_after(0, vec![
        ".####.....",
        "..........",
        "..........",
        "..........",
    ]);

    game.verify_frame_after(100, vec![
        "..........",
        ".####.....",
        "..........",
        "..........",
    ]);

    game.is_fast_dropping();

    game.verify_frame_after(110, vec![
        "..........",
        "..........",
        ".####.....",
        "..........",
        "..........",
    ]);

    game.verify_frame_after(120, vec![
        "..........",
        "..........",
        "..........",
        ".####.....",
        "..........",
    ]);

    game.stop_fast_dropping();

    game.verify_frame_after(130, vec![
        "..........",
        "..........",
        "..........",
        ".####.....",
        "..........",
    ]);

    game.verify_frame_after(220, vec![
        "..........",
        "..........",
        "..........",
        "..........",
        ".####.....",
    ])

}



