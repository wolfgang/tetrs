use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::j_block;


#[test]
fn cursor_right_moves_brick_right_every_50_ms() {
    let mut game = TestableGame::init().with_drop_interval(5000).build();

    game.is_moving_right();
    game.verify_frame_after(10, vec![".####....."]);
    game.verify_frame_after(50, vec!["..####...."]);
    game.verify_frame_after(80, vec!["..####...."]);
    game.verify_frame_after(100, vec!["...####..."]);

    game.is_moving_left();
    game.verify_frame_after(150, vec!["..####...."]);
    game.verify_frame_after(200, vec![".####....."]);
}

#[test]
fn horizontal_movement_is_constrained_by_bounds() {
    let mut game = TestableGame::init().with_drop_interval(5000).build();

    game.is_moving_left();
    game.verify_frame_after(50, vec!["####......"]);
    game.verify_frame_after(100, vec!["####......"]);

    game.is_moving_right();
    game.tick(150);
    game.tick(200);
    game.tick(250);
    game.tick(300);
    game.tick(350);
    game.verify_frame_after(400, vec!["......####"]);
    game.verify_frame_after(450, vec!["......####"]);
}

#[test]
fn cannot_move_horizontally_when_on_ground() {
    let mut game = TestableGame::init()
        .with_drop_interval(500)
        .with_field_height(2)
        .build();

    game.verify_exact_frame_after(500, vec![
        "..........",
        ".####....."
    ]);

    game.is_moving_left();

    game.verify_exact_frame_after(550, vec![
        "..........",
        ".####....."
    ]);

    game.is_moving_right();

    game.verify_exact_frame_after(600, vec![
        "..........",
        ".####....."
    ]);

}

#[test]
fn j_block_moving_left() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![j_block()])
        .with_drop_interval(500)
        .build();

    game.verify_frame_after(1, vec![
        ".#........",
        ".###......",
        ".........."
    ]);

    game.is_rotating();

    game.verify_frame_after(150, vec![
        "..##......",
        "..#.......",
        "..#......."
    ]);

    game.stop_rotating();
    game.is_moving_left();

    game.verify_frame_after(160, vec![
        ".##.......",
        ".#........",
        ".#........"
    ]);

    game.verify_frame_after(210, vec![
        "##........",
        "#.........",
        "#........."
    ]);

    game.verify_frame_after(300, vec![
        "##........",
        "#.........",
        "#........."
    ]);

}
