use std::io::Write;

pub struct Game {
    last_drop_millis: u64,
    brick_row: u8,
    field_height: u8
}

impl Game {
    pub fn new() -> Game {
        Game { brick_row: 0, last_drop_millis: 0, field_height: 16 }
    }

    pub fn tick(&mut self, new_time_millis: u64) {
        while new_time_millis - self.last_drop_millis  >= 100 {
            if self.brick_row < self.field_height - 1 {
                self.brick_row += 1;
            }
            self.last_drop_millis = self.last_drop_millis + 100;
        }
    }

    pub fn render(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for row in 0..self.field_height {
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