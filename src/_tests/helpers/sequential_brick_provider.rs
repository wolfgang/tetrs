use crate::game::brick_provider::{BrickProvider, BrickDef};
use std::rc::Rc;
use std::cell::RefCell;

pub struct SequentialBrickProvider {
    sequence: Vec<BrickDef>,
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

    pub fn add(&mut self, bricklets: BrickDef) {
        self.sequence.push(bricklets);
    }
}

impl BrickProvider for SequentialBrickProvider {
    fn next(&mut self) -> BrickDef {
        let next = self.sequence[self.current_index].clone();
        self.current_index += 1;
        next
    }
}
