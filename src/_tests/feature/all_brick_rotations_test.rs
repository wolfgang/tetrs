use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::*;

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

    game.is_rotating(true);

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
fn rotations_for_l_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![l_block()])
        .build();

    game.verify_frame_after(0, vec![
        ".#........",
        ".###......"
    ]);

    game.is_rotating(true);

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

#[test]
fn rotations_for_j_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![j_block()])
        .build();

    game.verify_frame_after(0, vec![
        "...#......",
        ".###......"
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "..#.......",
        "..#.......",
        "..##......"
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".###......",
        ".#........"
    ]);

    game.verify_frame_after(450, vec![
        ".##.......",
        "..#.......",
        "..#......."
    ]);

    game.verify_frame_after(600, vec![
        "...#......",
        ".###......"
    ]);
}

#[test]
fn rotations_for_s_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![s_block()])
        .build();

    game.verify_frame_after(0, vec![
        "..##......",
        ".##......."
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "..#.......",
        "..##......",
        "...#......"
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        "..##......",
        ".##......."
    ]);

    game.verify_frame_after(450, vec![
        ".#........",
        ".##.......",
        "..#......."
    ]);

    game.verify_frame_after(600, vec![
        "..##......",
        ".##......."
    ]);
}


#[test]
fn rotations_for_t_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![t_block()])
        .build();

    game.verify_frame_after(0, vec![
        "..#.......",
        ".###......"
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "..#.......",
        "..##......",
        "..#......."
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".###......",
        "..#......."
    ]);

    game.verify_frame_after(450, vec![
        "..#.......",
        ".##.......",
        "..#......."
    ]);

    game.verify_frame_after(600, vec![
        "..#.......",
        ".###......"
    ]);
}

#[test]
fn rotations_for_z_block() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![z_block()])
        .build();

    game.verify_frame_after(0, vec![
        ".##.......",
        "..##......"
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "...#......",
        "..##......",
        "..#......."
    ]);

    game.verify_frame_after(300, vec![
        "..........",
        ".##.......",
        "..##......"
    ]);

    game.verify_frame_after(450, vec![
        "..#.......",
        ".##.......",
        ".#........"
    ]);

    game.verify_frame_after(600, vec![
        ".##.......",
        "..##......"
    ]);

}