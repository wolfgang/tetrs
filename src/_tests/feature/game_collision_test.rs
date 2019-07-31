use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn when_hitting_ground_spawn_another_brick_after_100_ms() {
    let mut game = TestableGame::init().with_field_height(4).build();

    game.is_moving_right();
    game.tick(100);
    game.tick(200);
    game.verify_exact_frame_after(300, vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_not_moving();
    game.verify_exact_frame_after(400, vec![
        ".####.....",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_moving_left();
    game.verify_exact_frame_after(500, vec![
        "..........",
        "####......",
        "..........",
        "....####.."
    ]);
}

#[test]
fn when_hitting_another_brick_brick_stops_right_there() {
    let mut game = TestableGame::init().with_field_height(4).build();


    game.is_moving_right();
    game.tick(100);
    game.tick(200);
    game.verify_exact_frame_after(300, vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_not_moving();
    game.tick(400);
    game.tick(500);
    game.verify_exact_frame_after(600, vec![
        "..........",
        "..........",
        ".####.....",
        "....####.."
    ]);

    game.verify_exact_frame_after(700, vec![
        ".####.....",
        "..........",
        ".####.....",
        "....####.."
    ]);
}
