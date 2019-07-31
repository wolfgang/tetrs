use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

#[test]
fn when_hitting_ground_spawn_another_brick_after_100_ms() {
    let mut game = Game::init().with_field_height(4).build();
    let mut renderer = ToStringRenderer::with_height(4);

    game.tick(100);
    game.tick(200);
    game.tick(300);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        "..........",
        ".####....."
    ]);

    game.tick(400);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        ".####.....",
        "..........",
        "..........",
        ".####....."
    ]);

    game.tick(500);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        ".####.....",
        "..........",
        ".####....."
    ]);

}