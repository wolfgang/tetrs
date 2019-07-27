use std::io::Write;

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for row in 0..16 {
            if row == 0 {
                writer.write(b"| ####     |\n")?;
            }
            else {
                writer.write(b"|          |\n")?;
            }
        }
        writer.write(b"------------\n")?;
        Ok(())
    }
}