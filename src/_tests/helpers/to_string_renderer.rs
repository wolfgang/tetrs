use crate::game::trenderer::TRenderer;
use std::collections::HashMap;
use crate::game::brick_factory::*;

pub struct ToStringRenderer {
    pub frame: Vec<Vec<char>>,
    width: usize,
    use_brick_type_encoding: bool,
    brick_type_to_char: HashMap<u8, char>
}

impl ToStringRenderer {
    pub fn default() -> ToStringRenderer {
        Self::new(10, 16)
    }

    pub fn with_height(height: usize) -> ToStringRenderer {
        Self::new(10, height)
    }

    pub fn new(width: usize, height: usize) -> ToStringRenderer {
        let mut brick_type_to_char = HashMap::new();
        brick_type_to_char.insert(I_BLOCK, 'i');
        brick_type_to_char.insert(O_BLOCK, 'o');
        brick_type_to_char.insert(T_BLOCK, 't');
        brick_type_to_char.insert(J_BLOCK, 'j');
        brick_type_to_char.insert(S_BLOCK, 's');
        brick_type_to_char.insert(Z_BLOCK, 'z');
        brick_type_to_char.insert(L_BLOCK, 'l');

        ToStringRenderer {
            frame: vec![Vec::with_capacity(width); height],
            width,
            use_brick_type_encoding: false,
            brick_type_to_char
        }
    }

    pub fn use_brick_type_encoding(&mut self, b: bool) {
        self.use_brick_type_encoding = b;
    }

    pub fn assert_frame(&self, expected_frame: Vec<&str>) {
        self.assert_frame_internal(expected_frame, false);
    }

    pub fn assert_frame_exact(&self, expected_frame: Vec<&str>) {
        self.assert_frame_internal(expected_frame, true);
    }

    fn encode_brick_type(&self, brick_type: u8) -> char {
        if !self.use_brick_type_encoding { return '#' }
        return *self.brick_type_to_char.get(&brick_type).unwrap_or(&'#')
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

    fn draw_bricklet_at(&mut self, x: u8, y: u8, brick_type: u8) {
        self.frame[y as usize][x as usize] = self.encode_brick_type(brick_type)
    }
}