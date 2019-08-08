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
#[test]
fn lines_above_vanished_lines_drop_down() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field(vec![
            "..........",
            "#......###",
            "#....#####",
            ".########..",
        ])
        .build();

    game.verify_frame_after(0, vec![
        ".####.....",
        "#......###",
        "#....#####",
        ".########.",
    ]);

    game.verify_frame_after(100, vec![
        "..........",
        "#####..###",
        "#....#####",
        ".########.",
    ]);

    game.verify_frame_after(200, vec![
        "..........",
        "#......###",
        "##########",
        ".########.",
    ]);

    game.verify_frame_after(300, vec![
        ".####.....",
        "..........",
        "#......###",
        ".########.",
    ]);

}


#[test]
fn two_vanishing_lines() {
    let mut game = TestableGame::init()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field(vec![
            "........##",
            "..........",
            "###.######",
            "#....#....",
            "###.######",
        ])
        .with_drop_interval(200)
        .build();

    game.verify_frame_after(0, vec![
        ".####...##",
        "..........",
        "###.######",
        "#....#....",
        "###.######",
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "...#....##",
        "...#......",
        "##########",
        "#..#.#....",
        "###.######",
    ]);


    game.verify_frame_after(200, vec![
        "........##",
        "...#......",
        "##########",
        "#..#.#....",
        "##########",
    ]);

    game.verify_frame_after(400, vec![
        ".####.....",
        "..........",
        "........##",
        "...#......",
        "#..#.#...."
    ]);

}