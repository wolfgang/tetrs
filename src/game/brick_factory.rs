use crate::game::brick_provider::BrickDef;

const I_BLOCK: u8 = 1;
const O_BLOCK: u8 = 2;
const T_BLOCK: u8 = 3;
const J_BLOCK: u8 = 4;

// ####
pub fn i_block() -> BrickDef {
    BrickDef {
        brick_type: I_BLOCK,
        bricklets:
        vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(2, 0), (2, 1), (2, 2), (2, 3)]
        ],
    }
}

// ##
// ##
pub fn o_block() -> BrickDef {
    BrickDef {
        brick_type: O_BLOCK,
        bricklets: vec![vec![(0, 0), (1, 0), (0, 1), (1, 1)]],
    }
}

// ###
//  #
pub fn t_block_flipped() -> BrickDef {
    BrickDef {
        brick_type: T_BLOCK,
        bricklets: vec![vec![(0, 0), (1, 0), (2, 0), (1, 1)]],
    }
}

// ###
// #
pub fn j_block_flipped() -> BrickDef {
    BrickDef {
        brick_type: J_BLOCK,
        bricklets: vec![vec![(0, 0), (1, 0), (2, 0), (0, 1)]],
    }
}

// #
// ###
pub fn j_block() -> BrickDef {
    BrickDef {
        brick_type: J_BLOCK,
        bricklets:
        vec![
            vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            // .##
            // .#.
            // .#.
            vec![(1, 0), (2, 0), (1, 1), (1, 2)]
        ],
    }
}