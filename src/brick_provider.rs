use std::rc::Rc;

pub type BrickProviderRef = Rc<dyn BrickProvider>;

pub trait BrickProvider {
    fn next(&self) -> Vec<(u8, u8)>;
}

pub struct SingleBrickProvider {}

impl SingleBrickProvider {
    pub fn new_rc() -> BrickProviderRef {
        Rc::new(Self {})
    }
}

impl BrickProvider  for SingleBrickProvider {
    fn next(&self) -> Vec<(u8, u8)> {
        vec![(0, 0), (1, 0), (2, 0), (3, 0)]
    }
}