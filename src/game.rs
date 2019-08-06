pub mod tinput;
pub mod trenderer;
pub mod brick_provider;
pub mod brick_factory;

use tinput::{TInputRef, TInputNull};
use trenderer::TRenderer;
use brick_provider::{SingleBrickProvider, BrickProviderRef};
use crate::game::brick_provider::BrickDef;

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
            last_rotation_millis: 0,
            active_brick: Brick { x: 1, y: 0, phase: 0, brick_def: bricklets },
            input: self.input.clone(),
            brick_provider: self.brick_provider.clone(),
        }
    }
}

struct Brick {
    x: i8,
    y: u8,
    phase: usize,
    brick_def: BrickDef,
}

impl Brick {
    pub fn all_bricklets<F>(&self, condition: F) -> bool where F: Fn(usize, usize) -> bool {
        self.all_bricklets_at(self.phase, condition)
    }

    pub fn all_next_bricklets<F>(&self, condition: F) -> bool where F: Fn(usize, usize) -> bool {
        self.all_bricklets_at(self.next_phase(), condition)
    }

    pub fn current_bricklets(&self) -> Vec<(usize, usize)> {
        self.bricklets_at(self.phase)
    }

    pub fn goto_next_phase(&mut self) {
        self.phase = self.next_phase();
    }

    fn next_phase(&self) -> usize {
        (self.phase + 1) % self.brick_def.bricklets.len()
    }

    fn all_bricklets_at<F>(&self, phase: usize, condition: F) -> bool where F: Fn(usize, usize) -> bool {
        for (x, y) in self.bricklets_at(phase) {
            if !condition(x, y) { return false; }
        }
        true
    }

    fn bricklets_at(&self, phase: usize) -> Vec<(usize, usize)> {
        self.brick_def.bricklets[phase].iter().map(|(bx, by)| {
            ((self.x + *bx as i8) as usize, (self.y + *by) as usize)
        }).collect()
    }
}

pub struct Game {
    field_height: u8,
    field_width: u8,
    drop_interval: u16,
    last_drop_millis: u64,
    last_move_millis: u64,
    last_rotation_millis: u64,
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
        self.rotate_brick(now_millis);
        self.move_brick_horizontally(now_millis);
        self.drop_brick(now_millis)
    }

    pub fn render(&self, renderer: &mut dyn TRenderer) {
        renderer.clear();
        self.render_field(renderer);
        self.render_active_brick(renderer)
    }

    fn rotate_brick(&mut self, now_millis: u64) {
        if now_millis - self.last_rotation_millis >= 150 &&
            self.input.borrow().wants_to_rotate() &&
            self.can_rotate()
        {
            self.last_rotation_millis = now_millis;
            self.active_brick.goto_next_phase();
        }
    }

    fn move_brick_horizontally(&mut self, now_millis: u64) {
        let speed = self.get_horizontal_move_speed(now_millis);
        if speed != 0 {
            self.last_move_millis = now_millis;
            self.active_brick.x = self.active_brick.x + speed;
        }
    }

    fn drop_brick(&mut self, now_millis: u64) -> () {
        if self.is_time_to_drop(now_millis) {
            if self.can_drop() {
                self.active_brick.y += 1;
                self.last_drop_millis = now_millis;
            } else {
                self.last_drop_millis = now_millis;

                for (x, y) in self.active_brick.current_bricklets() {
                    self.field[y][x] = 1;
                }

                self.active_brick.x = 1;
                self.active_brick.y = 0;
                self.active_brick.brick_def = self.brick_provider.borrow_mut().next();
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
        now_millis - self.last_drop_millis >= self.drop_interval as u64
    }

    fn is_time_to_move(&self, now_millis: u64) -> bool {
        self.active_brick.y < self.field_height - 1 && now_millis - self.last_move_millis >= 50
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

    fn render_field(&self, renderer: &mut dyn TRenderer) {
        for (y, row) in self.field.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col != 0 {
                    renderer.draw_bricklet_at(x as u8, y as u8, 0)
                }
            }
        }
    }

    fn render_active_brick(&self, renderer: &mut dyn TRenderer) -> () {
        for (x, y) in self.active_brick.current_bricklets() {
            renderer.draw_bricklet_at(x as u8, y as u8, 0);
        }
    }
}