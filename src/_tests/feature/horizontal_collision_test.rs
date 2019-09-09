use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::*;

#[test]
fn moving_right_is_blocked_by_existing_brick() {
    let mut game = TestableGame::init()
        .with_brick_type_encoding()
        .with_brick_sequence(vec![o_block()])
        .with_field(vec![
            "..........",
            "...iiii...",
            "...iiii...",
            "...i......"])
        .build();

    game.verify_frame_after(1, vec![
        ".oo.......",
        ".ooiiii...",
        "...iiii...",
        "...i......"
    ]);

    game.is_moving_right();
    game.verify_frame_after(50, vec![
        ".oo.......",
        ".ooiiii...",
        "...iiii...",
        "...i......"
    ])
}

#[test]
fn moving_left_is_blocked_by_existing_brick() {
    let mut game = TestableGame::init()
        .with_brick_type_encoding()
        .with_brick_sequence(vec![o_block()])
        .with_field(vec![
            "..........",
            "i.........",
            "i.........",
            "i........."])
        .build();

    game.verify_frame_after(1, vec![
        ".oo.......",
        "ioo.......",
        "i.........",
        "i........."
    ]);

    game.is_moving_left();
    game.verify_frame_after(50, vec![
        ".oo.......",
        "ioo.......",
        "i.........",
        "i........."
    ]);

}