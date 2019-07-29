use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;
use crate::tinput::TInput;


struct TInputAlwaysGoingRight {}

impl TInput for TInputAlwaysGoingRight {
    fn wants_to_move_left(&self) -> bool {
        false
    }

    fn wants_to_move_right(&self) -> bool {
        true
    }
}



#[test]
fn cursor_right_moves_brick_right() {
    let mut game = Game::init().with_drop_interval(1000).build();
    let mut renderer = ToStringRenderer::default();

    // pressing right
    game.tick(50);

    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ".####.....",
        "..........",
        "..........",
    ]);
}