use crate::game::trenderer::TRenderer;
use crate::game::brick::Brick;
use crate::game::GameState;

pub struct GameRenderer {}

impl GameRenderer {
    pub fn render(&self, t_renderer: &mut dyn TRenderer, game_state: &GameState) {
        t_renderer.clear();
        self.render_field(t_renderer, &game_state.field);
        self.render_active_brick(t_renderer, &game_state.active_brick)
    }

    fn render_field(&self, t_renderer: &mut dyn TRenderer, field: &Vec<Vec<u8>>) {
        for (y, row) in field.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col != 0 {
                    t_renderer.draw_bricklet_at(x as u8, y as u8, *col)
                }
            }
        }
    }

    fn render_active_brick(&self, t_renderer: &mut dyn TRenderer, active_brick: &Brick) -> () {
        for (x, y) in active_brick.current_bricklets() {
            t_renderer.draw_bricklet_at(x as u8, y as u8, active_brick.brick_type());
        }
    }
}



