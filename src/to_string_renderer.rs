pub struct ToStringRenderer {
    pub frame_buffer: Vec<Vec<char>>,
    width: usize,
}

impl ToStringRenderer {
    pub fn new(width: usize, height: usize) -> ToStringRenderer {
        let frame_buffer = vec![Vec::with_capacity(width); height ];

        ToStringRenderer {
            frame_buffer,
            width,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.frame_buffer.len() {
            self.frame_buffer[i] = vec!['.'; self.width];
        }
    }

    pub fn frame_buffer_strings(&self) -> Vec<String> {
        self.frame_buffer
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }
}