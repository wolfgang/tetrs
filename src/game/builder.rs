use crate::game::tinput::{TInputRef, TInputNull};
use crate::game::brick_provider::{BrickProviderRef, SingleBrickProvider};
use crate::game::Game;

#[derive(Clone)]
pub struct GameBuilder {
    pub field_height: u8,
    pub initial_field: Vec<Vec<u8>>,
    pub current_time_millis: u64,
    pub drop_interval: u16,
    pub input: TInputRef,
    pub brick_provider: BrickProviderRef,
}

impl GameBuilder {
    pub fn init() -> GameBuilder {
        GameBuilder {
            field_height: 16,
            initial_field: Vec::new(),
            drop_interval: 100,
            current_time_millis: 0,
            input: TInputNull::new_rc(),
            brick_provider: SingleBrickProvider::new_rc(),
        }
    }

    pub fn with_field_height(&mut self, field_height: u8) -> &mut Self {
        self.field_height = field_height;
        self
    }

    pub fn with_field(&mut self, field: Vec<Vec<u8>>) -> &mut Self {
        self.initial_field = field;
        self
    }

    pub fn with_now_millis(&mut self, current_time_millis: u64) -> &mut Self {
        self.current_time_millis = current_time_millis;
        self
    }

    pub fn with_drop_interval(&mut self, drop_interval: u16) -> &mut Self {
        self.drop_interval = drop_interval;
        self
    }

    pub fn with_input(&mut self, input: TInputRef) -> &mut Self {
        self.input = input;
        self
    }

    pub fn with_brick_provider(&mut self, brick_provider: BrickProviderRef) -> &mut Self {
        self.brick_provider = brick_provider;
        self
    }

    pub fn build(&self) -> Game {
        Game::from_builder(self)
    }
}
