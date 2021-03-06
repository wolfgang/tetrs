use std::{cell::RefCell, rc::Rc};

pub type TInputRef = Rc<RefCell<dyn TInput>>;

pub trait TInput {
    fn wants_to_move_left(&self) -> bool;
    fn wants_to_move_right(&self) -> bool;
    fn wants_to_rotate(&self) ->  bool;
    fn wants_to_fast_drop(&self) -> bool;
    fn wants_to_insta_drop(&self) -> bool;

}

pub struct TInputNull {}

impl TInputNull {
    pub fn new_rc() -> TInputRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl TInput for TInputNull {
    fn wants_to_move_left(&self) -> bool {
        false
    }

    fn wants_to_move_right(&self) -> bool {
        false
    }

    fn wants_to_rotate(&self) -> bool {
        false
    }

    fn wants_to_fast_drop(&self) -> bool {
        false
    }

    fn wants_to_insta_drop(&self) -> bool {
        false
    }
}