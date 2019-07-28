use console::Term;
use tetrs::game::Game;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unreachable_code)]
fn main() -> std::io::Result<()> {

    let field_height = 16;
    let mut game = Game::init()
        .with_now_millis(get_now_millis())
        .with_field_height(field_height)
        .with_drop_interval(500)
        .build();


    let  rendered_lines = field_height + 1;

    let mut stdout = Term::stdout();
    stdout.clear_screen()?;

    let mut last_frame_millis = get_now_millis();

    loop {
        let now_millis = get_now_millis();
        if now_millis - last_frame_millis >= 100 {
            game.tick(get_now_millis());
            last_frame_millis = now_millis;
            stdout.clear_last_lines(rendered_lines as usize)?;
            game.render_to_console(&mut stdout).unwrap();
        }
    }

    Ok(())
}

fn get_now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}