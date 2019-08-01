use std::rc::Rc;
use std::cell::RefCell;

pub type BrickProviderRef = Rc<RefCell<dyn BrickProvider>>;
pub type Bricklets = Vec<(u8, u8)>;



pub trait BrickProvider {
    fn next(&mut self) -> Vec<(u8, u8)>;
}

pub struct SingleBrickProvider {}

impl SingleBrickProvider {
    pub fn new_rc() -> BrickProviderRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl BrickProvider  for SingleBrickProvider {
    fn next(&mut self) -> Vec<(u8, u8)> {
        vec![(0, 0), (1, 0), (2, 0), (3, 0)]
    }
}

//pub struct RandomBrickProvider {
//    bricks: Vec<Bricklets>
//}