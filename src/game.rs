use std::io::Write;

pub struct GameBuilder {
    field_height: u8,
    current_time_millis: u64,
    drop_interval: u16
}

impl GameBuilder {
    pub fn init() -> GameBuilder {
        GameBuilder {
            field_height: 16,
            drop_interval: 100,
            current_time_millis: 0,
        }
    }

    pub fn with_field_height(&mut self, field_height: u8) -> &mut Self {
        self.field_height = field_height;
        self
    }

    pub fn with_now_millis(&mut self, current_time_millis: u64) -> &mut Self {
        self.current_time_millis = current_time_millis;
        self
    }

    pub fn with_drop_interval(&mut self, drop_interval: u16) -> &mut Self {
        self.drop_interval = drop_interval;
        self
    }
    pub fn build(&self) -> Game {
        Game {
            field_height: self.field_height,
            drop_interval: self.drop_interval,
            last_drop_millis: self.current_time_millis,
            brick_row: 0,
        }
    }
}

pub struct Game {
    field_height: u8,
    drop_interval: u16,
    last_drop_millis: u64,
    brick_row: u8,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::init()
    }

    pub fn default() -> Game {
        Self::init().build()
    }

    pub fn tick(&mut self, new_time_millis: u64) {
        let current_row = self.brick_row;
        while new_time_millis - self.last_drop_millis >= self.drop_interval as u64 {
            if self.brick_row < self.field_height - 1 {
                self.brick_row += 1;
            }
            self.last_drop_millis = self.last_drop_millis + self.drop_interval as u64;
        }

        if self.brick_row > current_row {
            self.last_drop_millis = new_time_millis;
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