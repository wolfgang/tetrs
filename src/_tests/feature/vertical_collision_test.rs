use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, j_block_flipped, t_block_flipped};

#[test]
fn when_hitting_ground_spawn_another_brick_after_100_ms() {
    let mut game = TestableGame::init()
        .with_field_height(4)
        .with_brick_sequence(vec![j_block_flipped(), i_block()])
        .build();

    game.tick(100);
    game.verify_exact_frame_after(200, vec![
        "..........",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(250, vec![
        "..........",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(300, vec![
        ".####.....",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(400, vec![
        "..........",
        ".####.....",
        ".###......",
        ".#........"
    ]);
}

#[test]
fn when_hitting_another_brick_brick_spawn_another_brick_after_100ms() {
    let mut game = TestableGame::init().
        with_field_height(5)
        .with_brick_sequence(vec![j_block_flipped(), i_block(), t_block_flipped()])
        .build();
    game.tick(100);
    game.tick(300);
    game.verify_exact_frame_after(400, vec![
        "..........",
        "..........",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(450, vec![
        "..........",
        "..........",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(500, vec![
        ".####.....",
        "..........",
        "..........",
        ".###......",
        ".#........"
    ]);

    game.tick(600);
    game.verify_exact_frame_after(700, vec![
        "..........",
        "..........",
        ".####.....",
        ".###......",
        ".#........"
    ]);

    game.verify_exact_frame_after(800, vec![
        ".###......",
        "..#.......",
        ".####.....",
        ".###......",
        ".#........"
    ]);
}
