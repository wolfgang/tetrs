use std::{rc::Rc, cell::RefCell};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::game::brick_factory::*;


pub type BrickProviderRef = Rc<RefCell<dyn BrickProvider>>;
pub type Bricklets = Vec<Vec<(u8, u8)>>;


pub trait BrickProvider {
    fn next(&mut self) -> Bricklets;
}

pub struct SingleBrickProvider {}

impl SingleBrickProvider {
    pub fn new_rc() -> BrickProviderRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl BrickProvider for SingleBrickProvider {
    fn next(&mut self) -> Bricklets { i_block() }
}

pub struct RandomBrickProvider {
    bricks: Vec<Bricklets>,
    rng: StdRng,
}

impl RandomBrickProvider {
    pub fn new_rc() -> BrickProviderRef {
        let bricks = vec![
            i_block(),
            o_block(),
            j_block_flipped(),
            j_block(),
            t_block_flipped()

        ];
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Rc::new(RefCell::new(Self { bricks, rng: StdRng::seed_from_u64(seed) }))
    }
}

impl BrickProvider for RandomBrickProvider {
    fn next(&mut self) -> Bricklets {
        let index = self.rng.gen_range(0, self.bricks.len());
        self.bricks[index].clone()
    }
}