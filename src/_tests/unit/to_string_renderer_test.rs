use crate::_tests::helpers::to_string_renderer::*;
use crate::game::trenderer::TRenderer;
use crate::game::brick_factory::*;

#[test]
fn clear_fills_buffer_with_dots() {
    let mut renderer = ToStringRenderer::new(2, 4);
    renderer.clear();

    renderer.assert_frame(
        vec![
            "..",
            "..",
            "..",
            ".."
        ]
    );
}

#[test]
fn draw_brick_at_draws_hash_at_given_position() {
    let mut renderer = ToStringRenderer::new(3, 4);
    renderer.clear();

    renderer.draw_bricklet_at(0, 0, 0);
    renderer.draw_bricklet_at(1, 0, 0);
    renderer.draw_bricklet_at(0, 1, 0);
    renderer.draw_bricklet_at(1, 2, 0);

    renderer.assert_frame(vec![
        "##.",
        "#..",
        ".#.",
        "..."
    ]);
}


#[test]
fn assert_frame_accepts_first_n_lines() {
    let mut renderer = ToStringRenderer::new(3, 4);
    renderer.clear();

    renderer.draw_bricklet_at(0, 1, 0);

    renderer.assert_frame(vec![
        "...",
        "#.."
    ]);
}

//if brick_type == I_BLOCK { return Color::SKYBLUE}
//if brick_type == O_BLOCK { return Color::RED}
//if brick_type == T_BLOCK { return Color::VIOLET}
//if brick_type == J_BLOCK { return Color::BLUE}
//if brick_type == S_BLOCK { return Color::GOLD}
//if brick_type == Z_BLOCK { return Color::LIME}
//if brick_type == L_BLOCK { return Color::YELLOW}


#[test]
fn use_brick_type_encodings() {
    let mut renderer = ToStringRenderer::new(4, 2);
    renderer.use_brick_type_encoding(true);
    renderer.clear();

    renderer.draw_bricklet_at(0, 0, I_BLOCK);
    renderer.draw_bricklet_at(1, 0, O_BLOCK);
    renderer.draw_bricklet_at(2, 0, T_BLOCK);
    renderer.draw_bricklet_at(3, 0, J_BLOCK);
    renderer.draw_bricklet_at(0, 1, S_BLOCK);
    renderer.draw_bricklet_at(1, 1, Z_BLOCK);
    renderer.draw_bricklet_at(2, 1, L_BLOCK);

    renderer.assert_frame(vec![
        "iotj",
        "szl."
    ]);

}

