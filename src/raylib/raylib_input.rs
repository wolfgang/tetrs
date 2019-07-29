use raylib::RaylibHandle;
use raylib::consts::KEY_RIGHT;
use crate::tinput::{TInput, TInputRef};
use std::rc::Rc;
use std::cell::RefCell;

pub struct RaylibInput  {
    rl: Rc<RaylibHandle>
}

impl RaylibInput {
    pub fn new_ref(rl: Rc<RaylibHandle>) -> TInputRef {
        Rc::new(RefCell::new(Self { rl: rl.clone() }))
    }
}

impl TInput for RaylibInput {
    fn wants_to_move_left(&self) -> bool {
        false
    }

    fn wants_to_move_right(&self) -> bool {
        self.rl.is_key_down(KEY_RIGHT as i32)
    }
}