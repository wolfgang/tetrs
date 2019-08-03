use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::i_block;

#[ignore]
#[test]
fn rotating_simple_shape() {
    let mut game = TestableGame::init()
        .with_drop_interval(5000)
        .with_brick_sequence(vec![i_block()])
        .build();

    game.verify_frame_after(1, vec![
        ".####.....",
        ".........."
    ]);

    game.is_rotating();

    game.verify_frame_after(50, vec![
        "..#......",
        "..#.......",
        "..#.......",
        "..#.......",
        ".........."
    ]);

}