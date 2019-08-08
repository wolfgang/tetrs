use std::rc::Rc;
use std::cell::RefCell;
use crate::game::tinput::TInput;

pub type InputStubRef = Rc<RefCell<InputStub>>;

pub struct InputStub {
    moving_right: bool,
    moving_left: bool,
    rotating: bool,
    fast_dropping: bool
}

impl InputStub {
    pub fn new_rc() -> InputStubRef {
        Rc::new(RefCell::new(
            Self {
                moving_right: false,
                moving_left: false,
                rotating: false,
                fast_dropping: false
            }
        ))
    }

    pub fn is_moving_right(&mut self) {
        self.moving_right = true;
        self.moving_left = false;
    }

    pub fn is_moving_left(&mut self) {
        self.moving_left = true;
        self.moving_right = false;

    }

    pub fn is_not_moving(&mut self) {
        self.moving_left = false;
        self.moving_right = false;
    }

    pub fn is_rotating(&mut self) {
        self.rotating = true;
    }

    pub fn stop_rotating(&mut self) {
        self.rotating = false;
    }

    pub fn is_fast_dropping(&mut self) {
        self.fast_dropping = true;
    }

    pub fn stop_fast_dropping(&mut self) {
        self.fast_dropping = false;
    }

}

impl TInput for InputStub {
    fn wants_to_move_left(&self) -> bool {
        self.moving_left
    }

    fn wants_to_move_right(&self) -> bool {
        self.moving_right
    }

    fn wants_to_rotate(&self) -> bool {
        self.rotating
    }

    fn wants_to_fast_drop(&self) -> bool {
        self.fast_dropping
    }

}
