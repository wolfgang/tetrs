use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::*;

#[test]
fn more_complex_bricks() {
    let mut game = TestableGame::init()
        .with_field_height(5)
        .with_brick_sequence(vec![
            j_block_flipped(),
            j_block_flipped(),
            i_block()])
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
