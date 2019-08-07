use crate::game::brick::Brick;
use crate::game::tinput::TInputRef;
use crate::game::brick_provider::BrickProviderRef;
use crate::game::builder::GameBuilder;

const FIELD_WIDTH: usize = 10;

pub struct GameState {
    pub(super) last_drop_millis: u64,
    pub(super) last_move_millis: u64,
    pub(super) last_rotation_millis: u64,
    pub(super) field: Vec<Vec<u8>>,
    pub(super) active_brick: Brick,
    pub(super) drop_interval: u16,
    pub(super) input: TInputRef,
    pub(super) brick_provider: BrickProviderRef,
}

impl GameState {
    pub(super) fn from_game_builder(builder: &GameBuilder) -> Self {
        let mut field = builder.initial_field.clone();
        if builder.initial_field.len() == 0 {
            field = vec![vec![0; FIELD_WIDTH]; builder.field_height as usize];
        }

        Self {
            input: builder.input.clone(),
            brick_provider: builder.brick_provider.clone(),
            drop_interval: builder.drop_interval,
            last_drop_millis: builder.current_time_millis,
            last_move_millis: 0,
            last_rotation_millis: 0,
            field,
            active_brick: Brick::new(builder.brick_provider.clone()),
        }
    }


    pub(super) fn rotate_brick(&mut self, now_millis: u64) {
        if self.is_time_to_rotate(now_millis) &&
            self.input.borrow().wants_to_rotate() &&
            self.can_rotate()
        {
            self.last_rotation_millis = now_millis;
            self.active_brick.goto_next_phase();
        }
    }

    pub(super) fn move_brick_horizontally(&mut self, now_millis: u64) {
        let speed = self.get_horizontal_move_speed(now_millis);
        if speed != 0 {
            self.last_move_millis = now_millis;
            self.active_brick.move_by(speed, 0);
        }
    }

    pub(super) fn drop_brick(&mut self, now_millis: u64) -> () {
        if self.is_time_to_drop(now_millis) {
            self.last_drop_millis = now_millis;
            if self.can_drop() {
                self.active_brick.move_by(0, 1);
            } else {
                self.spawn_next_brick();
                self.check_vanishing_lines();
            }
        }
    }

    fn check_vanishing_lines(&mut self) {
        let mut valid_rows = self.field.iter().fold(
            Vec::with_capacity(self.field.len()),
            |mut acc, row| {
                if row.iter().any(|val| *val == 0) { acc.push(row.clone()); }
                acc
            });

        let mut new_field = vec![vec![0; FIELD_WIDTH]; self.field.len() - valid_rows.len()];
        new_field.append(&mut valid_rows);
        self.field = new_field;
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

    fn spawn_next_brick(&mut self) {
        for (x, y) in self.active_brick.current_bricklets() {
            self.field[y][x] = self.active_brick.brick_type();
        }
        self.active_brick.reset(self.brick_provider.clone());
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
        now_millis - self.last_move_millis >= 50
    }

    fn is_time_to_rotate(&self, now_millis: u64) -> bool {
        now_millis - self.last_rotation_millis >= 150
    }

    fn can_move_to(&self, offset: i32) -> bool {
        self.active_brick.all_bricklets(|x, y| { self.is_empty_cell(x as i32 + offset, y) })
    }

    fn is_empty_cell(&self, x: i32, y: usize) -> bool {
        x >= 0
            && (x as usize) < FIELD_WIDTH
            && y < self.field.len()
            && self.field[y][x as usize] == 0
    }
}
