use crate::gfx::trenderer::TRenderer;

pub struct ToStringRenderer {
    pub frame: Vec<Vec<char>>,
    width: usize,
}

impl ToStringRenderer {
    pub fn default() -> ToStringRenderer {
        Self::new(10, 16)
    }

    pub fn with_height(height: usize) -> ToStringRenderer {
        Self::new(10, height)
    }

    pub fn new(width: usize, height: usize) -> ToStringRenderer {
        ToStringRenderer {
            frame: vec![Vec::with_capacity(width); height],
            width,
        }
    }

    pub fn assert_frame(&self, expected_frame: Vec<&str>) {
        self.assert_frame_internal(expected_frame, false);
    }

    pub fn assert_frame_exact(&self, expected_frame: Vec<&str>) {
        self.assert_frame_internal(expected_frame, true);
    }

    fn assert_frame_internal(&self, expected_frame: Vec<&str>, exact: bool) {
        let max_checked_lines = if exact { self.frame.len() } else { expected_frame.len() };
        assert_eq!(
            self.frame_as_strings()[0 .. max_checked_lines].join("\n"),
            expected_frame.join("\n")
        )
    }


    fn frame_as_strings(&self) -> Vec<String> {
        self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }
}

impl TRenderer for ToStringRenderer {
    fn clear(&mut self) {
        for i in 0..self.frame.len() {
            self.frame[i] = vec!['.'; self.width];
        }
    }

    fn draw_bricklet_at(&mut self, x: u8, y: u8) {
        self.frame[y as usize][x as usize] = '#'
    }
}