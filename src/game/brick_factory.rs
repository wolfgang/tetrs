use crate::game::brick_provider::Bricklets;

// ####
pub fn i_block() -> Bricklets {
    vec![(0, 0), (1, 0), (2, 0), (3, 0)]
}

// ##
// ##
pub fn o_block() -> Bricklets {
    vec![(0, 0), (1, 0), (0, 1), (1, 1)]
}

// ###
//  #
pub fn  t_block_flipped() -> Bricklets {
    vec![(0, 0), (1, 0), (2, 0), (1, 1)]
}

// ####
// #
pub fn j_block_flipped() -> Bricklets {
    vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]
}