use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{j_block_flipped, o_block, i_block};

#[ignore]
#[test]
fn moving_right_is_blocked_by_existing_brick() {
    let mut game = TestableGame::init()
        .with_field_height(4)
        .with_brick_sequence(vec![j_block_flipped(), i_block(), o_block()])
        .build();

    game.is_moving_right();
    game.tick(100);
    game.verify_frame_after(200, vec![
        "..........",
        "..........",
        "...####...",
        "...#......"
    ]);

    game.tick(300);
    game.verify_frame_after(400, vec![
        "..........",
        "...####...",
        "...####...",
        "...#......"
    ]);

    game.is_not_moving();

    game.verify_frame_after(500, vec![
        "##........",
        "######....",
        "..####....",
        "..#......."
    ]);
}