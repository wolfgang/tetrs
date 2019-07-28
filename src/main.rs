use console::Term;
use tetrs::game::Game;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unreachable_code)]
fn main() -> std::io::Result<()> {
    let mut stdout = Term::stdout();
//    stdout.write_line("Hello, world!");

    let field_height = 16;
    let mut game = Game::init()
        .with_now_millis(get_now_millis())
        .with_field_height(field_height)
        .build();


    let mut rendered_lines = field_height + 1;
    loop {
        stdout.clear_last_lines(rendered_lines as usize)?;
        game.tick(get_now_millis());
        rendered_lines = game.render(&mut stdout).unwrap();

    }

    Ok(())
}

fn get_now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}