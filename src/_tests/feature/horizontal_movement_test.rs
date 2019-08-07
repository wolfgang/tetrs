use crate::_tests::helpers::testable_game::TestableGame;

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
