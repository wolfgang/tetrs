use std::rc::Rc;

use raylib::{RaylibHandle, Color};
use crate::game::trenderer::TRenderer;
use crate::game::brick_factory::{I_BLOCK, O_BLOCK, T_BLOCK, J_BLOCK, S_BLOCK};

const BRICKLET_SIZE: i32 = 32;

pub struct RaylibRenderer {
    rl: Rc<RaylibHandle>,
    width: usize,
    height: usize,
}

impl<'a> RaylibRenderer {
    pub fn new(rl: Rc<RaylibHandle>, width: usize, height: usize) -> RaylibRenderer {
        RaylibRenderer { rl: rl.clone(), width, height }
    }

    fn brick_color_for(brick_type: u8) -> Color {
        if brick_type == I_BLOCK { return Color::SKYBLUE}
        if brick_type == O_BLOCK { return Color::RED}
        if brick_type == T_BLOCK { return Color::VIOLET}
        if brick_type == J_BLOCK { return Color::BLUE}
        if brick_type == S_BLOCK { return Color::GOLD}

        return Color::WHITE;
    }

}

impl TRenderer for RaylibRenderer {
    fn clear(&mut self) {
        self.rl.clear_background(Color::BLACK);
        self.rl.draw_rectangle(
            BRICKLET_SIZE,
            BRICKLET_SIZE,
            BRICKLET_SIZE * self.width as i32,
            BRICKLET_SIZE * self.height as i32,
            Color::GREEN);
    }

    fn draw_bricklet_at(&mut self, x: u8, y: u8, _brick_type: u8) {
        self.rl.draw_rectangle(
            (x +1) as i32 * BRICKLET_SIZE,
            (y + 1) as i32 * BRICKLET_SIZE,
            BRICKLET_SIZE,
            BRICKLET_SIZE,
            Self::brick_color_for(_brick_type));
    }

}