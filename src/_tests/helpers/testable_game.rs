use crate::_tests::helpers::input_stub::{InputStubRef, InputStub};
use crate::game::{Game, GameBuilder};
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

pub struct TestableGameBuilder {
    game_builder: GameBuilder,
}

impl TestableGameBuilder {
    pub fn new() -> Self {
        TestableGameBuilder {
            game_builder: GameBuilder::init()
        }
    }

    pub fn with_field_height(&mut self, field_height: u8) -> &mut Self {
        self.game_builder = self.game_builder.with_field_height(field_height).clone();
        self
    }

    pub fn with_drop_interval(&mut self, drop_interval: u16) -> &mut Self {
        self.game_builder = self.game_builder.with_drop_interval(drop_interval).clone();
        self
    }

    pub fn build(&mut self) -> TestableGame {
        let input = InputStub::new_rc();
        TestableGame {
            input: input.clone(),
            game: self.game_builder.with_input(input.clone()).build(),
            renderer: ToStringRenderer::with_height(self.game_builder.field_height as usize)
        }
    }
}


pub struct TestableGame {
    input: InputStubRef,
    game: Game,
    renderer: ToStringRenderer,
}

impl TestableGame {
    pub fn init() -> TestableGameBuilder {
        TestableGameBuilder::new()
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

    pub fn verify_exact_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.game.render(&mut self.renderer);
        self.renderer.assert_frame_exact(expected_frame);
    }

    pub fn verify_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.game.render(&mut self.renderer);
        self.renderer.assert_frame(expected_frame);
    }

}