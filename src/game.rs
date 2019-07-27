use std::io::Write;

pub struct Game {
    time_millis: u64,
    brick_row: u8,
}

impl Game {
    pub fn new() -> Game {
        Game { brick_row: 0, time_millis: 0 }
    }

    pub fn tick(&mut self, delta_millis: u64) {
        self.time_millis += delta_millis;
        if self.time_millis > 100 { self.brick_row += 1; }
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