pub trait TRenderer {
    fn clear(&mut self);
    fn draw_bricklet_at(&mut self, x: u8, y: u8);
}