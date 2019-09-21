use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::brick_factory::{i_block, z_block};

#[test]
fn single_line_vanishes() {
    let mut game = TestableGame::init()
        .with_brick_type_encoding()
        .with_brick_sequence(vec![i_block(), z_block()])
        .with_field(vec![
            "..........",
            "..........",
            "o....zzool",
        ])
        .build();

    game.verify_frame_after(0, vec![
        ".iiii.....",
        "..........",
        "o....zzool",
    ]);

    game.tick(100);
    game.verify_frame_after(200, vec![
        "..........",
        "..........",
        "oiiiizzool",
    ]);

    game.verify_frame_after(300, vec![
        ".zz.......",
        "..zz......",
        "..........",
    ]);
}

#[test]
fn lines_above_vanished_lines_drop_down() {
    let mut game = TestableGame::init()
        .with_brick_type_encoding()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field(vec![
            "..........",
            "o......zzl",
            "o....zzllz",
            ".jjiiiiss.",
        ])
        .build();

    game.verify_frame_after(0, vec![
        ".iiii.....",
        "o......zzl",
        "o....zzllz",
        ".jjiiiiss.",
    ]);

    game.verify_frame_after(100, vec![
        "..........",
        "oiiii..zzl",
        "o....zzllz",
        ".jjiiiiss.",
    ]);

    game.verify_frame_after(200, vec![
        "..........",
        "o......zzl",
        "oiiiizzllz",
        ".jjiiiiss.",
    ]);

    game.verify_frame_after(300, vec![
        ".iiii.....",
        "..........",
        "o......zzl",
        ".jjiiiiss.",
    ]);
}

#[test]
fn two_vanishing_lines() {
    let mut game = TestableGame::init()
        .with_brick_type_encoding()
        .with_brick_sequence(vec![i_block(), i_block()])
        .with_field(vec![
            "........oo",
            "..........",
            "jjj.zzzlll",
            "i....o....",
            "zzo.llljjs",
        ])
        .with_drop_interval(200)
        .build();

    game.verify_frame_after(0, vec![
        ".iiii...oo",
        "..........",
        "jjj.zzzlll",
        "i....o....",
        "zzo.llljjs",
    ]);

    game.is_rotating(true);

    game.verify_frame_after(150, vec![
        "...i....oo",
        "...i......",
        "jjjizzzlll",
        "i..i.o....",
        "zzo.llljjs",
    ]);


    game.verify_frame_after(200, vec![
        "........oo",
        "...i......",
        "jjjizzzlll",
        "i..i.o....",
        "zzoillljjs",
    ]);

    game.verify_frame_after(400, vec![
        ".iiii.....",
        "..........",
        "........oo",
        "...i......"
    ]);
}