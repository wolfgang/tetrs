use crate::_tests::helpers::to_string_renderer::*;
use crate::game::Game;

#[test]
#[test]
fn render_initially_shows_field_with_one_brick() {
    let mut renderer = ToStringRenderer::default();
    let game = Game::default();

    game.render(&mut renderer);

    renderer.assert_frame_exact(vec![
        ".####.....",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
    ])
}

#[test]
fn every_100_ms_since_last_drop_the_brick_drops_down_one_row() {
    let mut game = Game::default();
    let mut renderer = ToStringRenderer::default();

    game.tick(50);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ".####.....",
        "..........",
        "..........",
    ]);

    game.tick(105);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####.....",
        "..........",
    ]);


    game.tick(160);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.tick(205);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        "..........",
        ".####.....",
    ]);
}


#[test]
fn after_brick_reaches_bottom_it_stays_there() {
    let mut game = Game::init().with_field_height(3).build();
    let mut renderer = ToStringRenderer::with_height(3);
    game.tick(100);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        ".####.....",
        "..........",
    ]);

    game.tick(200);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        ".####.....",
    ]);

    game.tick(250);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        ".####.....",
    ]);

}

#[test]
fn can_use_game_builder_to_override_initial_time() {
    let mut game = Game::init()
        .with_now_millis(5000)
        .build();
    let mut renderer = ToStringRenderer::default();

    game.tick(5000 + 100);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####.....",
        "..........",
        "..........",
        "..........",
        ".........."
    ]);
}


#[test]
fn can_use_game_builder_to_override_drop_interval() {
    let mut game = Game::init()
        .with_drop_interval(250)
        .build();
    let mut renderer = ToStringRenderer::default();

    game.tick(150);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ".####.....",
        "..........",
    ]);

    game.tick(250);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        ".####....."
    ]);
}



