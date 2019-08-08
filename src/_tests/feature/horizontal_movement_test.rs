use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn cursor_right_moves_brick_right_every_50_ms() {
    let mut game = TestableGame::init().with_drop_interval(5000).build();

    game.is_moving_right();
    game.verify_frame_after(10, vec![".####....."]);
    game.verify_frame_after(100, vec!["..####...."]);
    game.verify_frame_after(180, vec!["..####...."]);
    game.verify_frame_after(200, vec!["...####..."]);

    game.is_moving_left();
    game.verify_frame_after(300, vec!["..####...."]);
    game.verify_frame_after(400, vec![".####....."]);
}

#[test]
fn horizontal_movement_is_constrained_by_bounds() {
    let mut game = TestableGame::init().with_drop_interval(5000).build();

    game.is_moving_left();
    game.verify_frame_after(100, vec!["####......"]);
    game.verify_frame_after(200, vec!["####......"]);

    game.is_moving_right();
    game.tick(300);
    game.tick(400);
    game.tick(500);
    game.tick(600);
    game.tick(700);
    game.verify_frame_after(800, vec!["......####"]);
    game.verify_frame_after(900, vec!["......####"]);
}
