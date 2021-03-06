use crate::_tests::helpers::input_stub::{InputStubRef, InputStub};
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;
use crate::_tests::helpers::sequential_brick_provider::SequentialBrickProvider;

use crate::game::{Game, {builder:: GameBuilder}};
use crate::game::brick_provider::BrickDef;
use crate::_tests::helpers::brick_type::brick_char_to_type;

pub struct TestableGameBuilder {
    game_builder: GameBuilder,
    use_brick_type_encoding: bool
}

impl TestableGameBuilder {
    pub fn new() -> Self {
        TestableGameBuilder {
            game_builder: GameBuilder::init(),
            use_brick_type_encoding: false
        }
    }

    pub fn with_field_height(&mut self, field_height: u8) -> &mut Self {
        self.game_builder = self.game_builder.with_field_height(field_height).clone();
        self
    }

    pub fn with_field(&mut self, field_str: Vec<&'static str>) -> &mut Self {
        let field: Vec<Vec<u8>> = field_str.iter().map(|row| {
            row.chars()
                .map(|c| { brick_char_to_type(c) })
                .collect()
        }).collect();

        self.game_builder = self.game_builder.with_field(field).clone();
        self
    }

    pub fn with_drop_interval(&mut self, drop_interval: u16) -> &mut Self {
        self.game_builder = self.game_builder.with_drop_interval(drop_interval).clone();
        self
    }

    pub fn with_brick_sequence(&mut self, brick_sequence: Vec<BrickDef>) -> &mut Self {
        let brick_provider = SequentialBrickProvider::new_rc();
        for bricklets in brick_sequence {
            brick_provider.borrow_mut().add(bricklets)
        }
        self.game_builder = self.game_builder.with_brick_provider(brick_provider).clone();
        self
    }

    pub fn with_now_millis(&mut self, now_millis: u64) -> &mut Self {
        self.game_builder = self.game_builder.with_now_millis(now_millis).clone();
        self
    }

    pub fn with_brick_type_encoding(&mut self) -> &mut Self {
        self.use_brick_type_encoding = true;
        self
    }

    pub fn build(&mut self) -> TestableGame {
        let input = InputStub::new_rc();
        let mut renderer = ToStringRenderer::with_height(self.game_builder.field_height as usize);
        renderer.use_brick_type_encoding(self.use_brick_type_encoding);
        TestableGame {
            input: input.clone(),
            game: self.game_builder.with_input(input.clone()).build(),
            renderer
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

    pub fn default() -> Self {
        Self::init().build()
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

    pub fn is_rotating(&mut self, is: bool) {
        self.input.borrow_mut().is_rotating(is);
    }

    pub fn is_fast_dropping(&mut self, is: bool) {
        self.input.borrow_mut().is_fast_dropping(is);
    }

    pub fn is_insta_dropping(&mut self, is: bool) {
        self.input.borrow_mut().is_insta_dropping(is);
    }

    pub fn tick(&mut self, now: u64) {
        self.game.tick(now);
    }

    pub fn verify_exact_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.render();
        self.assert_frame_exact(expected_frame);
    }

    pub fn verify_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.render();
        self.renderer.assert_frame(expected_frame);
    }

    pub fn render(&mut self) {
        self.game.render(&mut self.renderer);
    }

    pub fn assert_frame_exact(&self, expected_frame: Vec<&str>) {
        self.renderer.assert_frame_exact(expected_frame);
    }
}