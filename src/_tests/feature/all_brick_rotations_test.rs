use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, j_block};

#[test]
fn rotations_for_i_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![i_block()])
        .build();

    game.verify_frame_after(0, vec![
        ".####.....",
        ".........."
    ]);

    game.is_rotating();

    game.verify_frame_after(150, vec![
        "...#......",
        "...#......",
        "...#......",
        "...#......"
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".####.....",
    ]);


    game.verify_frame_after(450, vec![
        "..#.......",
        "..#.......",
        "..#.......",
        "..#......."
    ]);

    game.verify_frame_after(600, vec![
        ".####.....",
        ".........."
    ]);

}

#[test]
fn rotations_for_j_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![j_block()])
        .build();

    game.verify_frame_after(0, vec![
        ".#........",
        ".###......"
    ]);

    game.is_rotating();

    game.verify_frame_after(150, vec![
        "..##......",
        "..#.......",
        "..#......."
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".###......",
        "...#......"
    ]);

    game.verify_frame_after(450, vec![
        "..#.......",
        "..#.......",
        ".##......."
    ]);

    game.verify_frame_after(600, vec![
        ".#........",
        ".###......"
    ]);

}