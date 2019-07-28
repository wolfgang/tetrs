pub struct ToStringRenderer {
    pub frame_buffer: Vec<String>,
    width: u8,
}

impl ToStringRenderer {
    pub fn new(width: u8, height: u8) -> ToStringRenderer {
        let mut frame_buffer: Vec<String> = Vec::with_capacity(height as usize);

        for _ in 0..height as usize {
            frame_buffer.push(String::with_capacity(width as usize))
        }

        ToStringRenderer {
            frame_buffer,
            width
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.frame_buffer.len() {
            self.frame_buffer[i] = ".".repeat(self.width as usize)
        }
    }

    pub fn frame_buffer_strings(&self) -> Vec<String> {
        self.frame_buffer.clone()
    }
}