use crate::_tests::helpers::testable_game::TestableGame;
use crate::_tests::helpers::sequential_brick_provider::SequentialBrickProvider;

#[test]
fn more_complex_bricks() {
    let brick_provider = SequentialBrickProvider::new_rc();
    brick_provider.borrow_mut().add(vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]);
    brick_provider.borrow_mut().add(vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]);
    brick_provider.borrow_mut().add(vec![(0, 0), (1, 0), (2, 0), (3, 0)]);

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
