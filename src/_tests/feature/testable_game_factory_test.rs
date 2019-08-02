use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::o_block;

#[test]
fn can_initialize_field() {
    let mut game = TestableGame::init()
        .with_field_height(4)
        .with_brick_sequence(vec![o_block()])
        .with_field(vec![
            "..........",
            "......####",
            "...####...",
            "...#......"])
        .build();

    game.verify_frame_after(1, vec![
        ".##.......",
        ".##...####",
        "...####...",
        "...#......"
    ])
}