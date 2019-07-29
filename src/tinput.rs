
pub trait TInput {
    fn wants_to_move_left(&self) -> bool;
    fn wants_to_move_right(&self) -> bool;
}
