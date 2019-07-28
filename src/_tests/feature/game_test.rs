use crate::_tests::to_string_renderer::ToStringRenderer;
use crate::game::Game;
use crate::trenderer::TRenderer;

#[test]
#[test]
fn render_initially_shows_field_with_one_brick() {
    let mut renderer = ToStringRenderer::new(10, 5);
    let game = Game::init()
        .with_field_height(5)
        .build();

    renderer.clear();

    game.render(&mut renderer);

    renderer.assert_frame(vec![
        ".####.....",
        "..........",
        "..........",
        "..........",
        "..........",
    ])
}
