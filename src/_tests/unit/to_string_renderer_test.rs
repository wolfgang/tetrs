use crate::to_string_renderer::ToStringRenderer;

#[test]
fn clear_renders_all_blanks() {
    let mut renderer = ToStringRenderer::new(2, 4);
    renderer.clear();

    assert_eq!(renderer.frame_buffer_strings(), vec!["..", "..", "..", ".."]);
}