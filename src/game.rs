use std::io::Write;


pub struct GameConfig {
    pub field_height: u8
}

pub struct Game {
    last_drop_millis: u64,
    brick_row: u8,
    field_height: u8
}

impl Game {
    pub fn new() -> Game {
        Game::with_config(&GameConfig {field_height: 16})
    }


    pub fn with_config(config: & GameConfig) -> Game {
        Game {
            brick_row: 0,
            last_drop_millis: 0,
            field_height: config.field_height }
    }

    pub fn tick(&mut self, new_time_millis: u64) {
        while new_time_millis - self.last_drop_millis  >= 100 {
            if self.brick_row < self.field_height - 1 {
                self.brick_row += 1;
            }
            self.last_drop_millis = self.last_drop_millis + 100;
        }
    }

    pub fn render(&self, writer: &mut dyn Write) -> std::io::Result<u8> {
        for row in 0..self.field_height {
            if row == self.brick_row {
                writer.write(b"| ####     |\n")?;
            } else {
                writer.write(b"|          |\n")?;
            }
        }
        writer.write(b"------------\n")?;
        Ok(self.field_height + 1)
    }
}