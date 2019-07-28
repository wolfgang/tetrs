use raylib::{RaylibHandle, Color};
use crate::trenderer::TRenderer;

const BRICKLET_SIZE: i32 = 32;

pub struct RaylibRenderer<'a> {
    rl: &'a RaylibHandle,
    width: usize,
    height: usize,
}

impl<'a> RaylibRenderer<'a> {
    pub fn new(rl: &'a RaylibHandle, width: usize, height: usize) -> RaylibRenderer {
        RaylibRenderer { rl, width, height }
    }
}

impl TRenderer for RaylibRenderer<'_> {
    fn clear(&mut self) {
        self.rl.clear_background(Color::DARKBLUE);
        self.rl.draw_rectangle(
            BRICKLET_SIZE,
            BRICKLET_SIZE,
            BRICKLET_SIZE * self.width as i32,
            BRICKLET_SIZE * self.height as i32,
            Color::GREEN);
    }

    fn draw_bricklet_at(&mut self, x: u8, y: u8) {
        self.rl.draw_rectangle(
            (x +1) as i32 * BRICKLET_SIZE,
            (y + 1) as i32 * BRICKLET_SIZE,
            BRICKLET_SIZE,
            BRICKLET_SIZE,
            Color::RED);
    }
}