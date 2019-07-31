use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;
use crate::_tests::helpers::input_stub::InputStub;

#[test]
fn when_hitting_ground_spawn_another_brick_after_100_ms() {
    let input = InputStub::new_rc();
    let mut game = Game::init()
        .with_input(input.clone())
        .with_field_height(4).build();
    let mut renderer = ToStringRenderer::with_height(4);

    input.borrow_mut().is_moving_right();
    game.tick(100);
    game.tick(200);
    game.tick(300);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    input.borrow_mut().is_not_moving();
    game.tick(400);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        ".####.....",
        "..........",
        "..........",
        "....####.."
    ]);

    input.borrow_mut().is_moving_left();
    game.tick(500);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "####......",
        "..........",
        "....####.."
    ]);

}

#[test]
fn when_hitting_another_brick_brick_stops_right_there() {
    let input = InputStub::new_rc();
    let mut game = Game::init()
        .with_input(input.clone())
        .with_field_height(4).build();
    let mut renderer = ToStringRenderer::with_height(4);

    input.borrow_mut().is_moving_right();
    game.tick(100);
    game.tick(200);
    game.tick(300);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    input.borrow_mut().is_not_moving();
    game.tick(400);
    game.tick(500);
    game.tick(600);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        "..........",
        "..........",
        ".####.....",
        "....####.."
    ]);

    game.tick(700);
    game.render(&mut renderer);
    renderer.assert_frame_exact(vec![
        ".####.....",
        "..........",
        ".####.....",
        "....####.."
    ]);

}