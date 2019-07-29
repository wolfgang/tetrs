use tetrs::game::Game;
use std::time::{SystemTime, UNIX_EPOCH};
use raylib;
use tetrs::gfx::raylib_renderer::RaylibRenderer;
use tetrs::raylib_input::RaylibInput;
use std::rc::Rc;

fn main() {
    let field_height = 16;

    let rl = raylib::init()
        .size(800, 600)
        .title("TetRS")
        .build();

    let rl_ref = Rc::new(rl);

    let mut renderer = RaylibRenderer::new(rl_ref.clone(), 10, field_height as usize);

    let mut game = Game::init()
        .with_now_millis(get_now_millis())
        .with_field_height(field_height)
        .with_drop_interval(1000)
        .with_input(RaylibInput::new_ref(rl_ref.clone()))
        .build();

    while !rl_ref.window_should_close() {
        rl_ref.begin_drawing();
        game.tick(get_now_millis());
        game.render(&mut renderer);
        rl_ref.end_drawing();
    }
}

fn get_now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}