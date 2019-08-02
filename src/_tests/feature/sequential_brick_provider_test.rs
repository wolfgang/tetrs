use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, j_block_flipped};

#[test]
fn use_sequential_provider() {
    let mut game = TestableGame::init()
        .with_field_height(3)
        .with_brick_sequence(vec![i_block(), j_block_flipped()])
        .build();

    game.verify_exact_frame_after(100, vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.tick(200);
    game.verify_exact_frame_after(300, vec![
        ".####.....",
        ".#........",
        ".####....."
    ]);


}