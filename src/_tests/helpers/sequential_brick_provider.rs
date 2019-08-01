use crate::brick_provider::BrickProvider;
use std::rc::Rc;
use std::cell::RefCell;

type Bricklets = Vec<(u8, u8)>;

pub struct SequentialBrickProvider {
    sequence: Vec<Bricklets>,
    current_index: usize,
}

impl SequentialBrickProvider {
    pub fn new_rc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            SequentialBrickProvider {
                sequence: Vec::with_capacity(10),
                current_index: 0,
            }))
    }

    pub fn add(&mut self, bricklets: Bricklets) {
        self.sequence.push(bricklets);
    }
}

impl BrickProvider for SequentialBrickProvider {
    fn next(&mut self) -> Vec<(u8, u8)> {
        let next = self.sequence[self.current_index].clone();
        self.current_index += 1;
        next
    }
}
