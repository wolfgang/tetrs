use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;

#[ignore]
#[test]
fn spawn_another_brick_on_top_after_first_hits_the_ground() {
    let mut game = Game::init().with_field_height(4).build();
    let mut renderer = ToStringRenderer::with_height(4);

    game.tick(100);
    game.tick(200);
    game.tick(300);
    game.tick(400);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        "..........",
        ".####....."
    ]);

    game.tick(500);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        ".####.....",
        "..........",
        "..........",
        ".####....."
    ]);

}