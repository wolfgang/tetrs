use crate::game::brick_provider::BrickDef;

pub struct Brick {
    pub x: i8,
    pub y: u8,
    phase: usize,
    brick_def: BrickDef,
}

impl Brick {
    pub fn new(brick_def: BrickDef) -> Self {
        Self {
            x: 1,
            y: 0,
            phase: 0,
            brick_def
        }
    }


    pub fn reset(&mut self, brick_def: BrickDef) {
        self.x = 1;
        self.y = 0;
        self.brick_def = brick_def;
        self.phase = 0;
    }

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

    pub fn brick_type(&self) -> u8 {
        self.brick_def.brick_type
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
