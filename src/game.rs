pub mod tinput;
pub mod trenderer;
pub mod brick_provider;
pub mod brick_factory;
pub mod brick;
pub mod builder;
pub mod renderer;
pub mod state;

use trenderer::TRenderer;
use brick::Brick;
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
        let mut field = builder.initial_field.clone();
        if builder.initial_field.len() == 0 {
            field = vec![vec![0; 10]; builder.field_height as usize];
        }

        Self {
            renderer: GameRenderer {},
            state: GameState {
                input: builder.input.clone(),
                brick_provider: builder.brick_provider.clone(),
                drop_interval: builder.drop_interval,
                last_drop_millis: builder.current_time_millis,
                last_move_millis: 0,
                last_rotation_millis: 0,
                field,
                active_brick: Brick::new(builder.brick_provider.clone())

            },
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