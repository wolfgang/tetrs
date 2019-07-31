use crate::_tests::helpers::input_stub::{InputStubRef, InputStub};
use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

pub struct TestableGame {
    input: InputStubRef,
    game: Game,
    renderer: ToStringRenderer,
}

impl TestableGame {
    pub fn with_field_height(field_height: u8) -> Self {
        let input = InputStub::new_rc();
        Self {
            input: input.clone(),
            game: Game::init()
                .with_input(input.clone())
                .with_field_height(field_height).build(),
            renderer: ToStringRenderer::with_height(field_height as usize),
        }
    }

    pub fn is_moving_left(&mut self) {
        self.input.borrow_mut().is_moving_left();
    }

    pub fn is_moving_right(&mut self) {
        self.input.borrow_mut().is_moving_right();
    }

    pub fn is_not_moving(&mut self) {
        self.input.borrow_mut().is_not_moving();
    }

    pub fn tick(&mut self, now: u64) {
        self.game.tick(now);
    }

    pub fn verify_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.game.render(&mut self.renderer);
        self.renderer.assert_frame_exact(expected_frame);
    }
}