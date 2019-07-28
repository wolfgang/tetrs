use crate::trenderer::TRenderer;

pub struct ToStringRenderer {
    pub frame: Vec<Vec<char>>,
    width: usize,
}

impl ToStringRenderer {
    pub fn new(width: usize, height: usize) -> ToStringRenderer {
        ToStringRenderer {
            frame: vec![Vec::with_capacity(width); height],
            width,
        }
    }

    pub fn assert_frame(&self, expected_frame: Vec<&str>) {
        assert_eq!(
            self.frame_as_strings().join("\n"),
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