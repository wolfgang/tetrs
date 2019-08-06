use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, j_block};

#[test]
fn rotating_simple_shape() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![i_block()])
        .build();

    game.verify_frame_after(1, vec![
        ".####.....",
        ".........."
    ]);

    game.is_rotating();

    game.verify_frame_after(30, vec![
        ".####.....",
        ".........."
    ]);

    game.verify_frame_after(50, vec![
        "...#......",
        "...#......",
        "...#......",
        "...#......",
        ".........."
    ]);

    game.verify_frame_after(100, vec![
        "..........",
        ".####.....",
        "..........",
        "..........",
        ".........."
    ]);

    game.verify_frame_after(150, vec![
        "..........",
        "...#......",
        "...#......",
        "...#......",
        "...#......"
    ]);
}

#[test]
fn rotating_brick_with_collision() {
    let mut game =  TestableGame::init()
        .with_field(vec![
            "..........",
            "..........",
            "..........",
            ".####.....",
            "..........",
            "..........",
        ])
        .with_brick_sequence(vec![j_block()])
        .build();

    game.verify_frame_after(100, vec![
        "..........",
        ".#........",
        ".###......",
        ".####....."
    ]);

    game.is_rotating();

    game.verify_frame_after(150, vec![
        "..........",
        ".#........",
        ".###......",
        ".####....."
    ]);

}