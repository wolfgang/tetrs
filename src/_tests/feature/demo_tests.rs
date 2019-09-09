use crate::game::brick_factory::*;
use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn after_brick_reaches_bottom_it_stays_there() {
    let mut game = TestableGame::init()
        .with_drop_interval(100)
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field_height(3)
        .build();

    game.verify_exact_frame_after(100, vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.verify_exact_frame_after(200, vec![
        "..........",
        "..........",
        ".####....."
    ]);

    game.verify_exact_frame_after(300, vec![
        ".####.....",
        "..........",
        ".####....."
    ]);
}
