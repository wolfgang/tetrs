use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, j_block};

#[test]
fn rotating_brick_with_collision_1() {
    let mut game = TestableGame::init()
        .with_field(vec![
            "..........",
            "..........",
            "..........",
            ".####.....",
            "..........",
            "..........",
        ])
        .with_drop_interval(500)
        .with_brick_sequence(vec![j_block()])
        .build();

    game.verify_frame_after(500, vec![
        "..........",
        ".#........",
        ".###......",
        ".####....."
    ]);

    game.is_rotating();

    game.verify_frame_after(700, vec![
        "..........",
        ".#........",
        ".###......",
        ".####....."
    ]);
}

#[test]
fn rotating_brick_with_collision_2() {
    let mut game = TestableGame::init()
        .with_field(vec![
            "..........",
            "....####..",
            "..........",
            "..........",
        ])
        .with_drop_interval(500)
        .with_brick_sequence(vec![j_block()])
        .build();

    game.verify_frame_after(1, vec![
        ".#........",
        ".#######..",
        "..........",
        "..........",
    ]);

    game.is_rotating();

    game.verify_frame_after(150, vec![
        "..##......",
        "..#.####..",
        "..#.......",
        "..........",
    ]);
}

#[test]
fn rotating_brick_with_collision_3() {
    let mut game = TestableGame::init()
        .with_field(vec![
            "..........",
            "..........",
            "..........",
            "..........",
            ".########."
        ])
        .with_brick_sequence(vec![i_block()])
        .with_drop_interval(500)
        .build();

    game.is_rotating();
    game.verify_frame_after(150, vec![
        "...#......",
        "...#......",
        "...#......",
        "...#......",
        ".########."
    ]);

    game.verify_frame_after(300, vec![
        ".####.....",
        "..........",
        "..........",
        "..........",
        ".########."
    ]);
}
