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

    pub fn draw_brick_at(&mut self, x: u8, y: u8) {
        self.frame_buffer[y as usize][x as usize] = '#'
    }

    pub fn assert_frame(&self, expected_frame: Vec<&str>) {
        assert_eq!(
            self.frame_buffer_strings().join("\n"),
            expected_frame.join("\n")
        )
    }

    pub fn frame_buffer_strings(&self) -> Vec<String> {
        self.frame_buffer
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }
}