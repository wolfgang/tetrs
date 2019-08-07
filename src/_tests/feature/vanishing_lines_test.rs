use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::i_block;

#[test]
fn single_line_vanishes() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field(vec![
            "..........",
            "..........",
            "#....#####",
        ])
        .build();

    game.verify_frame_after(0, vec![
        ".####.....",
        "..........",
        "#....#####",
    ]);

    game.tick(100);
    game.verify_frame_after(200,vec![
        "..........",
        "..........",
        "##########",
    ]);

    game.verify_frame_after(300,vec![
        ".####.....",
        "..........",
        "..........",
    ]);

}