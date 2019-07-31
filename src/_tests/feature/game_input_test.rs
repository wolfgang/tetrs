use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

use crate::tinput::TInput;
use std::rc::Rc;
use std::cell::RefCell;

type InputStubRef = Rc<RefCell<InputStub>>;

struct InputStub {
    moving_right: bool,
    moving_left: bool
}

impl InputStub {
    pub fn new_rc() -> InputStubRef {
        Rc::new(RefCell::new(
            Self {
                moving_right: false,
                moving_left: false
            }
        ))
    }
    
    pub fn is_moving_right(&mut self) {
        self.moving_right = true;
        self.moving_left = false;
    }

    pub fn is_moving_left(&mut self) {
        self.moving_left = true;
        self.moving_right = false;

    }
}

impl TInput for InputStub {
    fn wants_to_move_left(&self) -> bool {
        self.moving_left
    }

    fn wants_to_move_right(&self) -> bool {
        self.moving_right
    }
}

#[test]
fn cursor_right_moves_brick_right_every_50_ms() {
    let (mut game, input, mut renderer) = setup_game();

    input.borrow_mut().is_moving_right();

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

    input.borrow_mut().is_moving_left();

    game.tick(150);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["..####...."]);

    game.tick(200);
    game.render(&mut renderer);
    renderer.assert_frame(vec![".####....."]);
}

#[test]
fn horizontal_movement_is_constrained_by_bounds() {
    let (mut game, input, mut renderer) = setup_game();

    input.borrow_mut().is_moving_left();

    game.tick(50);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["####......"]);

    game.tick(100);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["####......"]);

    input.borrow_mut().is_moving_right();

    game.tick(150);
    game.tick(200);
    game.tick(250);
    game.tick(300);
    game.tick(350);
    game.tick(400);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["......####"]);

    game.tick(450);
    game.render(&mut renderer);
    renderer.assert_frame(vec!["......####"]);
}

#[test]
fn cannot_move_horizontally_when_on_ground() {
    let input = InputStub::new_rc();
    let mut game = Game::init()
        .with_input(input.clone())
        .with_drop_interval(100)
        .with_field_height(2)
        .build();

    let mut renderer = ToStringRenderer::with_height(2);

    game.tick(100);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        ".####....."
    ]);

    input.borrow_mut().is_moving_left();

    game.tick(150);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        ".####....."
    ]);

    input.borrow_mut().is_moving_right();

    game.tick(190);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        ".####....."
    ]);

}

fn setup_game() -> (Game, InputStubRef, ToStringRenderer) {
    let input = InputStub::new_rc();
    let game = Game::init()
        .with_input(input.clone())
        .with_drop_interval(5000)
        .build();

    (game, input, ToStringRenderer::default())
}
