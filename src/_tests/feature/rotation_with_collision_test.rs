use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, l_block};

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
        .with_brick_sequence(vec![l_block()])
        .build();

    game.verify_frame_after(500, vec![
        "..........",
        ".#........",
        ".###......",
        ".####....."
    ]);

    game.is_rotating(true);

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
        .with_brick_sequence(vec![l_block()])
        .build();

    game.verify_frame_after(1, vec![
        ".#........",
        ".#######..",
        "..........",
        "..........",
    ]);

    game.is_rotating(true);

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

    game.is_rotating(true);
    game.verify_frame_after(150, vec![
        "...#......",
        "...#......",
        "...#......",
        "...#......",
        ".########."
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".####.....",
        "..........",
        "..........",
        ".########."
    ]);
}

#[test]
fn j_block_moving_left() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![l_block()])
        .with_drop_interval(500)
        .build();

    game.verify_frame_after(1, vec![
        ".#........",
        ".###......",
        ".........."
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "..##......",
        "..#.......",
        "..#......."
    ]);

    game.is_rotating(false);
    game.is_moving_left();

    game.verify_frame_after(160, vec![
        ".##.......",
        ".#........",
        ".#........"
    ]);

    game.verify_frame_after(260, vec![
        "##........",
        "#.........",
        "#........."
    ]);

    game.verify_frame_after(360, vec![
        "##........",
        "#.........",
        "#........."
    ]);

}
