pub mod tinput;
pub mod trenderer;
pub mod brick_provider;
pub mod brick_factory;
pub mod brick;
pub mod builder;
pub mod renderer;

use tinput::TInputRef;
use trenderer::TRenderer;
use brick_provider::BrickProviderRef;
use brick::Brick;
use builder::GameBuilder;
use crate::game::renderer::GameRenderer;


#[derive(Default)]
pub struct GameState {
    last_drop_millis: u64,
    last_move_millis: u64,
    last_rotation_millis: u64
}

pub struct Game {
    field_height: u8,
    field_width: u8,
    drop_interval: u16,
    field: Vec<Vec<u8>>,
    active_brick: Brick,
    input: TInputRef,
    brick_provider: BrickProviderRef,
    renderer: GameRenderer,
    state: GameState,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::init()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let mut field = builder.initial_field.clone();
        let mut field_height = field.len() as u8;
        if field.len() == 0 {
            field = vec![vec![0; 10]; builder.field_height as usize];
            field_height = builder.field_height;
        }

        Self {
            field_width: 10,
            field_height,
            field,
            drop_interval: builder.drop_interval,
            active_brick: Brick::new(builder.brick_provider.borrow_mut().next()),
            input: builder.input.clone(),
            brick_provider: builder.brick_provider.clone(),
            renderer: GameRenderer {},
            state: GameState {
                last_drop_millis: builder.current_time_millis,
                ..Default::default()
            },
        }
    }

    pub fn tick(&mut self, now_millis: u64) {
        self.rotate_brick(now_millis);
        self.move_brick_horizontally(now_millis);
        self.drop_brick(now_millis)
    }

    pub fn render(&self, t_renderer: &mut dyn TRenderer) {
        self.renderer.render(t_renderer, &self.field, &self.active_brick);
    }

    fn rotate_brick(&mut self, now_millis: u64) {
        if now_millis - self.state.last_rotation_millis >= 150 &&
            self.input.borrow().wants_to_rotate() &&
            self.can_rotate()
        {
            self.state.last_rotation_millis = now_millis;
            self.active_brick.goto_next_phase();
        }
    }

    fn move_brick_horizontally(&mut self, now_millis: u64) {
        let speed = self.get_horizontal_move_speed(now_millis);
        if speed != 0 {
            self.state.last_move_millis = now_millis;
            self.active_brick.move_by(speed, 0);
        }
    }

    fn drop_brick(&mut self, now_millis: u64) -> () {
        if self.is_time_to_drop(now_millis) {
            if self.can_drop() {
                self.active_brick.move_by(0, 1);
                self.state.last_drop_millis = now_millis;
            } else {
                self.state.last_drop_millis = now_millis;

                for (x, y) in self.active_brick.current_bricklets() {
                    self.field[y][x] = self.active_brick.brick_type();
                }

                self.active_brick.reset(self.brick_provider.borrow_mut().next());
            }
        }
    }

    fn can_rotate(&self) -> bool {
        self.active_brick.all_next_bricklets(|x, y| {
            self.is_empty_cell(x as i32, y)
        })
    }

    fn get_horizontal_move_speed(&self, now_millis: u64) -> i8 {
        if !self.is_time_to_move(now_millis) { return 0; };
        if self.input.borrow().wants_to_move_right() && self.can_move_to(1) { return 1; }
        if self.input.borrow().wants_to_move_left() && self.can_move_to(-1) { return -1; }
        return 0;
    }

    fn can_drop(&self) -> bool {
        self.active_brick.all_bricklets(|x, y| {
            self.is_empty_cell(x as i32, y + 1)
        })
    }

    fn is_time_to_drop(&self, now_millis: u64) -> bool {
        now_millis - self.state.last_drop_millis >= self.drop_interval as u64
    }

    fn is_time_to_move(&self, now_millis: u64) -> bool {
        self.active_brick.is_above(self.field_height - 1) && now_millis - self.state.last_move_millis >= 50
    }

    fn can_move_to(&self, offset: i32) -> bool {
        self.active_brick.all_bricklets(|x, y| { self.is_empty_cell(x as i32 + offset, y) })
    }

    fn is_empty_cell(&self, x: i32, y: usize) -> bool {
        x >= 0
            && (x as usize) < self.field_width as usize
            && y < self.field_height as usize
            && self.field[y][x as usize] == 0
    }
}