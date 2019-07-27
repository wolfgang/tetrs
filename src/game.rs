use std::io::Write;

pub struct Game {
    brick_row: u8
}

impl Game {
    pub fn new() -> Game {
        Game { brick_row: 0 }
    }

    pub fn tick(&mut self, delta_millis: u64) {
        self.brick_row += 1;
    }

    pub fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for row in 0..16 {
            if row == self.brick_row {
                writer.write(b"| ####     |\n")?;
            } else {
                writer.write(b"|          |\n")?;
            }
        }
        writer.write(b"------------\n")?;
        Ok(())
    }
}