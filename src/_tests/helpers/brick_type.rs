use crate::game::brick_factory::*;
pub fn brick_type_to_char(brick_type: u8) -> char {
    match brick_type {
        I_BLOCK => { 'i' }
        O_BLOCK => { 'o' }
        T_BLOCK => { 't' }
        J_BLOCK => { 'j' }
        S_BLOCK => { 's' }
        Z_BLOCK => { 'z' }
        L_BLOCK => { 'l' }
        _ => '#'
    }
}

pub fn brick_char_to_type(c: char) -> u8 {
    match c {
        '.' => { 0 }
        'i' => { I_BLOCK }
        'o' => { O_BLOCK }
        't' => { T_BLOCK }
        'j' => { J_BLOCK }
        's' => { S_BLOCK }
        'z' => { Z_BLOCK }
        'l' => { L_BLOCK }
        _ => I_BLOCK
    }
}

