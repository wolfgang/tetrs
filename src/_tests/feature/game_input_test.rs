use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

use crate::tinput::{TInput, TInputRef};
use std::rc::Rc;
use std::cell::RefCell;

struct TInputAlwaysGoingRight {}

impl TInputAlwaysGoingRight {
    pub fn new_rc() -> TInputRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl TInput for TInputAlwaysGoingRight {
    fn wants_to_move_left(&self) -> bool {
        false
    }

    fn wants_to_move_right(&self) -> bool {
        true
    }
}


#[test]
fn cursor_right_moves_brick_right_every_50_ms() {
    let mut game = Game::init()
        .with_input(TInputAlwaysGoingRight::new_rc())
        .with_drop_interval(1000)
        .build();
    let mut renderer = ToStringRenderer::default();

    game.tick(10);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ".####.....",
    ]);

    game.tick(50);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..####....",
    ]);

    game.tick(80);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..####....",
    ]);

    game.tick(100);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "...####...",
    ]);

}