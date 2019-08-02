use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::*;

#[test]
fn moving_right_is_blocked_by_existing_brick() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![o_block()])
        .with_field(vec![
            "..........",
            "...####...",
            "...####...",
            "...#......"])
        .build();

    game.verify_frame_after(1, vec![
        ".##.......",
        ".######...",
        "...####...",
        "...#......"
    ]);

    game.is_moving_right();
    game.verify_frame_after(50, vec![
        ".##.......",
        ".######...",
        "...####...",
        "...#......"
    ])

}