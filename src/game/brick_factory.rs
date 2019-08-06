use crate::game::brick_provider::BrickDef;

pub const I_BLOCK: u8 = 1;
pub const O_BLOCK: u8 = 2;
pub const T_BLOCK: u8 = 3;
pub const J_BLOCK: u8 = 4;
pub const S_BLOCK: u8 = 5;
pub const Z_BLOCK: u8 = 6;
pub const L_BLOCK: u8 = 7;

pub fn i_block() -> BrickDef {
    BrickDef {
        brick_type: I_BLOCK,
        bricklets:
        vec![
            from_strings(vec!["####"]),
            from_strings(vec![
                "..#.",
                "..#.",
                "..#.",
                "..#."]),
            from_strings(vec![
                "....",
                "####"]),
            from_strings(vec![
                ".#..",
                ".#..",
                ".#..",
                ".#.."]),
        ],
    }
}

pub fn o_block() -> BrickDef {
    BrickDef {
        brick_type: O_BLOCK,
        bricklets: vec![
            from_strings(vec![
                "##",
                "##"])
        ],
    }
}

pub fn t_block() -> BrickDef {
    BrickDef {
        brick_type: T_BLOCK,
        bricklets: vec![
            from_strings(vec![
                ".#.",
                "###"
            ]),
            from_strings(vec![
                ".#.",
                ".##",
                ".#."
            ]),
            from_strings(vec![
                "...",
                "###",
                ".#."
            ]),
            from_strings(vec![
                ".#.",
                "##.",
                ".#."
            ])

        ],
    }
}

pub fn l_block() -> BrickDef {
    BrickDef {
        brick_type: L_BLOCK,
        bricklets:
        vec![
            from_strings(vec![
                "#..",
                "###"
            ]),
            from_strings(vec![
                ".##",
                ".#.",
                ".#."
            ]),
            from_strings(vec![
                "...",
                "###",
                "..#"
            ]),
            from_strings(vec![
                ".#.",
                ".#.",
                "##."
            ]),
        ],
    }
}

pub fn j_block() -> BrickDef {
    BrickDef {
        brick_type: J_BLOCK,
        bricklets:
        vec![
            from_strings(vec![
                "..#",
                "###"
            ]),
            from_strings(vec![
                ".#.",
                ".#.",
                ".##"
            ]),
            from_strings(vec![
                "...",
                "###",
                "#.."
            ]),
            from_strings(vec![
                "##.",
                ".#.",
                ".#."
            ]),
        ],
    }
}

pub fn s_block() -> BrickDef {
    BrickDef {
        brick_type: S_BLOCK,
        bricklets:
        vec![
            from_strings(vec![
                ".##",
                "##."
            ]),
            from_strings(vec![
                ".#.",
                ".##",
                "..#"
            ]),
            from_strings(vec![
                "...",
                ".##",
                "##."
            ]),
            from_strings(vec![
                "#.",
                "##",
                ".#"
            ]),
        ],
    }
}


pub fn z_block() -> BrickDef {
    BrickDef {
        brick_type: Z_BLOCK,
        bricklets:
        vec![
            from_strings(vec![
                "##.",
                ".##."
            ]),
            from_strings(vec![
                "..#",
                ".##",
                ".#."
            ]),
            from_strings(vec![
                "...",
                "##.",
                ".##"
            ]),
            from_strings(vec![
                ".#",
                "##",
                "#."
            ]),
        ],
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
pub fn l_block_flipped() -> BrickDef {
    BrickDef {
        brick_type: J_BLOCK,
        bricklets: vec![vec![(0, 0), (1, 0), (2, 0), (0, 1)]],
    }
}

fn from_strings(strings: Vec<&str>) -> Vec<(u8, u8)> {
    let mut result = Vec::with_capacity(8);
    for (y, row) in strings.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '#' { result.push((x as u8, y as u8)) }
        }
    }
    result
}
