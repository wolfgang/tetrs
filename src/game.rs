pub mod tinput;
pub mod trenderer;
pub mod brick_provider;
pub mod brick_factory;

use tinput::{TInputRef, TInputNull};
use trenderer::TRenderer;
use brick_provider::{SingleBrickProvider, BrickProviderRef};

#[derive(Clone)]
pub struct GameBuilder {
    pub field_height: u8,
    initial_field: Vec<Vec<u8>>,
    current_time_millis: u64,
    drop_interval: u16,
    input: TInputRef,
    brick_provider: BrickProviderRef,
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
        let bricklets = self.brick_provider.borrow_mut().next();
        let mut field = self.initial_field.clone();
        let mut field_height = field.len() as u8;
        if field.len() == 0 {
            field = vec![vec![0; 10]; self.field_height as usize];
            field_height = self.field_height;
        }

        Game {
            field_width: 10,
            field_height,
            field,
            drop_interval: self.drop_interval,
            last_drop_millis: self.current_time_millis,
            last_move_millis: 0,
            active_brick: Brick { x: 1, y: 0, width: 4, bricklets },
            input: self.input.clone(),
            brick_provider: self.brick_provider.clone(),
        }
    }
}

struct Brick {
    x: u8,
    y: u8,
    width: u8,
    bricklets: Vec<(u8, u8)>,
}

impl Brick {
    pub fn for_any_bricklet<F>(&self, condition: F) -> bool where F: Fn(usize, usize) -> bool {
        for (bx, by) in &self.bricklets {
            let x = (self.x + *bx) as usize;
            let y = (self.y + *by) as usize;

            if condition(x, y) { return true }
        }

        false
    }
}

pub struct Game {
    field_height: u8,
    field_width: u8,
    drop_interval: u16,
    last_drop_millis: u64,
    last_move_millis: u64,
    field: Vec<Vec<u8>>,
    active_brick: Brick,
    input: TInputRef,
    brick_provider: BrickProviderRef,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::init()
    }

    pub fn default() -> Game {
        Self::init().build()
    }

    pub fn tick(&mut self, now_millis: u64) {
        self.move_brick_horizontally(now_millis);
        self.drop_brick(now_millis)
    }

    pub fn render(&self, renderer: &mut dyn TRenderer) {
        renderer.clear();
        self.render_field(renderer);
        self.render_active_brick(renderer)
    }

    fn move_brick_horizontally(&mut self, now_millis: u64) {
        let speed = self.get_horizontal_move_speed(now_millis);
        if speed != 0 {
            self.last_move_millis = now_millis;
            self.active_brick.x = (self.active_brick.x as i8 + speed) as u8;
        }
    }

    fn drop_brick(&mut self, now_millis: u64) -> () {
        if self.is_time_to_drop(now_millis) {
            if self.can_drop() {
                self.active_brick.y += 1;
                self.last_drop_millis = now_millis;
            } else {
                self.last_drop_millis = now_millis;
                let x = self.active_brick.x as usize;
                let y = self.active_brick.y as usize;

                for (bx, by) in &self.active_brick.bricklets {
                    self.field[y + *by as usize][x + *bx as usize] = 1;
                }

                self.active_brick.x = 1;
                self.active_brick.y = 0;
                self.active_brick.bricklets = self.brick_provider.borrow_mut().next();
            }
        }
    }

    fn get_horizontal_move_speed(&self, now_millis: u64) -> i8 {
        if self.active_brick.y < self.field_height - 1 && now_millis - self.last_move_millis >= 50 {
            if self.input.borrow().wants_to_move_right() {
                if self.active_brick.for_any_bricklet(|x, y| {
                    x == self.field_width as usize - 1 || self.field[y][x + 1] != 0
                })
                {
                    return 0;
                }

                return 1;
            }
            if self.input.borrow().wants_to_move_left() && self.active_brick.x > 0 {
                return -1;
            }
        }
        0
    }

    fn can_drop(&self) -> bool {
        !self.active_brick.for_any_bricklet(|x, y| {
            y == self.field_height as usize - 1 || self.field[y + 1][x] != 0
        })
    }

    fn is_time_to_drop(&self, now_millis: u64) -> bool {
        now_millis - self.last_drop_millis >= self.drop_interval as u64
    }

    fn render_field(&self, renderer: &mut dyn TRenderer) {
        for (y, row) in self.field.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col != 0 {
                    renderer.draw_bricklet_at(x as u8, y as u8)
                }
            }
        }
    }

    fn render_active_brick(&self, renderer: &mut dyn TRenderer) -> () {
        for (bx, by) in &self.active_brick.bricklets {
            let x = self.active_brick.x + *bx;
            let y = self.active_brick.y + *by;
            renderer.draw_bricklet_at(x, y);
        }
    }
}