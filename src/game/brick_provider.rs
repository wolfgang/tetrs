use std::rc::Rc;
use std::cell::RefCell;

use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


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

impl BrickProvider for SingleBrickProvider {
    fn next(&mut self) -> Vec<(u8, u8)> {
        vec![(0, 0), (1, 0), (2, 0), (3, 0)]
    }
}

pub struct RandomBrickProvider {
    bricks: Vec<Bricklets>,
    rng: StdRng,
}

impl RandomBrickProvider {
    pub fn new_rc() -> BrickProviderRef {
        let bricks = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            vec![(0, 0), (1, 0), (2, 0), (1, 1)]
        ];
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Rc::new(RefCell::new(Self { bricks, rng: StdRng::seed_from_u64(seed) }))
    }
}

impl BrickProvider for RandomBrickProvider {
    fn next(&mut self) -> Vec<(u8, u8)> {
        let index = self.rng.gen_range(0, self.bricks.len());
        self.bricks[index].clone()
    }
}