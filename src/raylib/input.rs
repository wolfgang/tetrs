use std::{rc::Rc, cell::RefCell};

use raylib::RaylibHandle;
use raylib::consts::{KEY_RIGHT, KEY_LEFT, KEY_UP, KEY_DOWN, KEY_SPACE};

use crate::game::tinput::{TInput, TInputRef};

pub struct RaylibInput  {
    rl: Rc<RaylibHandle>
}

impl RaylibInput {
    pub fn new_rc(rl: Rc<RaylibHandle>) -> TInputRef {
        Rc::new(RefCell::new(Self { rl: rl.clone() }))
    }
}

impl TInput for RaylibInput {
    fn wants_to_move_left(&self) -> bool {
        self.rl.is_key_down(KEY_LEFT as i32)
    }

    fn wants_to_move_right(&self) -> bool {
        self.rl.is_key_down(KEY_RIGHT as i32)
    }

    fn wants_to_rotate(&self) -> bool {
        self.rl.is_key_down(KEY_UP as i32)
    }

    fn wants_to_fast_drop(&self) -> bool {
        self.rl.is_key_down(KEY_DOWN as i32)
    }

    fn wants_to_insta_drop(&self) -> bool {
        self.rl.is_key_down(KEY_SPACE as i32)
    }
}