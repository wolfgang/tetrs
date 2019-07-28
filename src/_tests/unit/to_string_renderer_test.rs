use crate::_tests::to_string_renderer::ToStringRenderer;
use crate::trenderer::TRenderer;

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

    renderer.draw_bricklet_at(0, 0);
    renderer.draw_bricklet_at(1, 0);
    renderer.draw_bricklet_at(0, 1);
    renderer.draw_bricklet_at(1, 2);

    renderer.assert_frame(vec![
        "##.",
        "#..",
        ".#.",
        "..."
    ]);
}

