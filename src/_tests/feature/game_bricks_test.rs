use crate::_tests::helpers::testable_game::TestableGame;
use crate::_tests::helpers::sequential_brick_provider::SequentialBrickProvider;
use crate::game::brick_factory::*;

#[test]
fn more_complex_bricks() {
    let brick_provider = SequentialBrickProvider::new_rc();
    // 2x   ####   1x ####
    //      #
    brick_provider.borrow_mut().add(j_block_flipped());
    brick_provider.borrow_mut().add(j_block_flipped());
    brick_provider.borrow_mut().add(i_block());

    let mut game = TestableGame::init()
        .with_field_height(5)
        .with_brick_provider(brick_provider.clone())
        .build();

    game.verify_exact_frame_after(1, vec![
        ".####.....",
        ".#........",
        "..........",
        "..........",
        ".........."
    ]);

    game.tick(100);
    game.tick(200);
    game.verify_exact_frame_after(300, vec![
        "..........",
        "..........",
        "..........",
        ".####.....",
        ".#........"
    ]);

    game.verify_exact_frame_after(400, vec![
        ".####.....",
        ".#........",
        "..........",
        ".####.....",
        ".#........"
    ]);

    game.verify_exact_frame_after(500, vec![
        "..........",
        ".####.....",
        ".#........",
        ".####.....",
        ".#........"
    ]);

    game.verify_exact_frame_after(600, vec![
        ".####.....",
        ".####.....",
        ".#........",
        ".####.....",
        ".#........"
    ]);

}
