use std::io::Write;

pub struct Game {

}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writer.write(b"HELLO1\n")?;
        writer.write(b"HELLO2\n")?;
        Ok(())
    }
}