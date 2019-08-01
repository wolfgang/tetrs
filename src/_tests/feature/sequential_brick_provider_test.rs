use crate::_tests::helpers::testable_game::TestableGame;
use crate::_tests::helpers::sequential_brick_provider::SequentialBrickProvider;

#[test]
fn use_sequential_provider() {
    let brick_provider = SequentialBrickProvider::new_rc();
    brick_provider.borrow_mut().add(vec![(0, 0), (1, 0)]);
    brick_provider.borrow_mut().add(vec![(2, 0), (3, 0)]);


    let mut game = TestableGame::init()
        .with_field_height(3)
        .with_brick_provider(brick_provider.clone())
        .build();

    game.verify_exact_frame_after(100, vec![
        "..........",
        ".##.......",
        "..........",
    ]);

    game.tick(200);
    game.verify_exact_frame_after(300, vec![
        "...##.....",
        "..........",
        ".##......."
    ]);


}