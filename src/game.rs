pub mod tinput;
pub mod trenderer;
pub mod brick_provider;
pub mod brick_factory;
pub mod brick;
pub mod builder;
pub mod renderer;
pub mod state;

use trenderer::TRenderer;
use builder::GameBuilder;
use renderer::GameRenderer;
use state::GameState;


pub struct Game {
    renderer: GameRenderer,
    state: GameState,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::init()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        Self {
            renderer: GameRenderer {},
            state: GameState::from_game_builder(builder)
        }
    }

    pub fn tick(&mut self, now_millis: u64) {
        self.state.rotate_brick(now_millis);
        self.state.move_brick_horizontally(now_millis);
        self.state.drop_brick(now_millis)
    }

    pub fn render(&self, t_renderer: &mut dyn TRenderer) {
        self.renderer.render(t_renderer, &self.state);
    }

}