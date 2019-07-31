use crate::trenderer::TRenderer;
use crate::tinput::{TInputRef, TInputNull};

pub struct GameBuilder {
    field_height: u8,
    current_time_millis: u64,
    drop_interval: u16,
    input: TInputRef,
}

impl GameBuilder {
    pub fn init() -> GameBuilder {
        GameBuilder {
            field_height: 16,
            drop_interval: 100,
            current_time_millis: 0,
            input: TInputNull::new_rc(),
        }
    }

    pub fn with_field_height(&mut self, field_height: u8) -> &mut Self {
        self.field_height = field_height;
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
        self.input = input.clone();
        self
    }

    pub fn build(&self) -> Game {
        Game {
            field_width: 10,
            field_height: self.field_height,
            drop_interval: self.drop_interval,
            last_drop_millis: self.current_time_millis,
            last_move_millis: 0,
            input: self.input.clone(),
            active_brick: Brick { x: 1, y: 0, width: 4},
            field: vec![vec![0; 10]; self.field_height as usize]
        }
    }
}

struct Brick {
    x: u8,
    y: u8,
    width: u8
}

pub struct Game {
    field_height: u8,
    field_width: u8,
    drop_interval: u16,
    last_drop_millis: u64,
    last_move_millis: u64,
    active_brick: Brick,
    input: TInputRef,
    field: Vec<Vec<u8>>
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

        for (y, row) in self.field.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col != 0 {
                    renderer.draw_bricklet_at(x as u8, y as u8)
                }
            }
        }

        for offset in 0 .. self.active_brick.width {
            renderer.draw_bricklet_at(self.active_brick.x + offset, self.active_brick.y);
        }
    }

fn move_brick_horizontally(&mut self, now_millis: u64) {
    let speed = self.get_horizontal_move_speed(now_millis);
    if speed != 0 {
        self.last_move_millis = now_millis;
        self.active_brick.x = (self.active_brick.x as i8 + speed) as u8;
    }
}

fn drop_brick(&mut self, now_millis: u64) -> () {
    if self.active_brick.y < self.field_height - 1 && self.is_time_to_drop(now_millis)
    {
        self.active_brick.y += 1;
        self.last_drop_millis = now_millis;
    }
    else if self.is_time_to_drop(now_millis) {
        self.last_drop_millis = now_millis;
        let x = self.active_brick.x as usize;
        let y = self.active_brick.y as usize;

        for offset in 0 .. self.active_brick.width as usize {
            self.field[y][x + offset] = 1;
        }

        self.active_brick.x = 1;
        self.active_brick.y = 0;
    }
}

fn get_horizontal_move_speed(&self, now_millis: u64) -> i8 {
    if self.active_brick.y < self.field_height - 1 && now_millis - self.last_move_millis >= 50 {
        if self.input.borrow().wants_to_move_right() &&
            self.active_brick.x < self.field_width - self.active_brick.width {
            return 1;
        }
        if self.input.borrow().wants_to_move_left() && self.active_brick.x > 0 {
            return -1;
        }
    }
    0
}

fn is_time_to_drop(&self, now_millis: u64) -> bool {
    now_millis - self.last_drop_millis >= self.drop_interval as u64
}

}