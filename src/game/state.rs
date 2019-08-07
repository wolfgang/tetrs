use crate::game::brick::Brick;
use crate::game::tinput::TInputRef;

const FIELD_WIDTH: usize = 10;

pub struct GameState {
    pub(super) last_drop_millis: u64,
    pub(super) last_move_millis: u64,
    pub(super) last_rotation_millis: u64,
    pub(super) field: Vec<Vec<u8>>,
    pub(super) active_brick: Brick,
    pub(super) drop_interval: u16,
    pub(super) input: TInputRef,
}

impl GameState {
    pub(super) fn can_drop(&self) -> bool {
        self.active_brick.all_bricklets(|x, y| {
            self.is_empty_cell(x as i32, y + 1)
        })
    }

    pub(super) fn is_time_to_drop(&self, now_millis: u64) -> bool {
        now_millis - self.last_drop_millis >= self.drop_interval as u64
    }

    pub(super) fn is_time_to_move(&self, now_millis: u64) -> bool {
        self.active_brick.is_above(self.field.len() as u8 - 1) &&
            now_millis - self.last_move_millis >= 50
    }

    pub(super) fn can_move_to(&self, offset: i32) -> bool {
        self.active_brick.all_bricklets(|x, y| { self.is_empty_cell(x as i32 + offset, y) })
    }

    pub(super) fn is_empty_cell(&self, x: i32, y: usize) -> bool {
        x >= 0
            && (x as usize) < FIELD_WIDTH
            && y < self.field.len()
            && self.field[y][x as usize] == 0
    }
}
