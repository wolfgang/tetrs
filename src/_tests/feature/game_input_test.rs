use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

use crate::tinput::TInput;
use std::rc::Rc;
use std::cell::RefCell;

struct StubbedInput {
    moving_right: bool
}

impl StubbedInput {
    pub fn new_rc() -> Rc<RefCell<StubbedInput>> {
        Rc::new(RefCell::new(
            Self {
                moving_right: false
            }
        ))
    }
    
    pub fn toggle_moving_right(&mut self, b: bool) {
        self.moving_right = b;
    }
}

impl TInput for StubbedInput {
    fn wants_to_move_left(&self) -> bool {
        false
    }

    fn wants_to_move_right(&self) -> bool {
        self.moving_right
    }
}


#[test]
fn cursor_right_moves_brick_right_every_50_ms() {
    let input = StubbedInput::new_rc();
    let mut game = Game::init()
        .with_input(input.clone())
        .with_drop_interval(1000)
        .build();
    let mut renderer = ToStringRenderer::default();

    input.borrow_mut().toggle_moving_right(true);


    game.tick(10);
    game.render(&mut renderer);
    renderer.assert_frame(vec![".####....."]);

    game.tick(50);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["..####...."]);

    game.tick(80);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["..####...."]);

    game.tick(100);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["...####..."]);

}