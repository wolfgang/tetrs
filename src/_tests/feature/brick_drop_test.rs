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
    game.is_rotating(true);
    game.verify_frame_after(150, vec![
        "..........",
        "...#......",
        "...#......",
        "...#......",
        "...#......",
    ]);

    game.is_rotating(false);

    game.verify_frame_after(200, vec![
        ".####.....",
        "...#......",
        "...#......",
        "...#......",
        "...#......",
    ]);
}

#[test]
fn drop_with_10_ms_interval_if_fast_drop_activated() {
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

    game.is_fast_dropping(true);

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

    game.is_fast_dropping(false);

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

#[test]
fn suspend_fast_drop_after_new_brick_spawns() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_drop_interval(100)
        .with_now_millis(1000)
        .with_field_height(4)
        .build();

    game.verify_frame_after(1000, vec![
        ".####.....",
        "..........",
        "..........",
        ".........."
    ]);

    game.is_fast_dropping(true);

    game.verify_frame_after(1010, vec![
        "..........",
        ".####.....",
        "..........",
        ".........."
    ]);

    game.verify_frame_after(1020, vec![
        "..........",
        "..........",
        ".####.....",
        ".........."
    ]);

    game.verify_frame_after(1030, vec![
        "..........",
        "..........",
        "..........",
        ".####.....",
    ]);

    game.verify_frame_after(1040, vec![
        ".####.....",
        "..........",
        "..........",
        ".####.....",
    ]);

    // After brick lands, next brick is not dropping as fast
    game.verify_frame_after(1050, vec![
        ".####.....",
        "..........",
        "..........",
        ".####.....",
    ]);

    game.verify_frame_after(1140, vec![
        "..........",
        ".####.....",
        "..........",
        ".####.....",
    ]);

    // Fast drop resumes here if still active per input
    game.verify_frame_after(1150, vec![
        "..........",
        "..........",
        ".####.....",
        ".####.....",
    ]);

}


