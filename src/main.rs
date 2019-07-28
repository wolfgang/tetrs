use console::Term;
use tetrs::game::Game;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unreachable_code)]
fn main() -> std::io::Result<()> {
    let mut stdout = Term::stdout();
//    stdout.write_line("Hello, world!");

    let mut game = Game::new();

    let mut number_of_lines = 16;

    game.init_time(get_now_millis());

    loop {
        stdout.clear_last_lines(number_of_lines as usize)?;
        game.tick(get_now_millis());
        number_of_lines = game.render(&mut stdout).unwrap();

    }

    Ok(())
}

fn get_now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}