use crate::game::brick_provider::Bricklets;

// ####
pub fn i_block() -> Bricklets {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (1, 1), (1, 2), (1, 3)]

    ]
}

// ##
// ##
pub fn o_block() -> Bricklets {
    vec![vec![(0, 0), (1, 0), (0, 1), (1, 1)]]
}

// ###
//  #
pub fn  t_block_flipped() -> Bricklets {
    vec![vec![(0, 0), (1, 0), (2, 0), (1, 1)]]
}

// ####
// #
pub fn j_block_flipped() -> Bricklets {
    vec![vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)]]
}