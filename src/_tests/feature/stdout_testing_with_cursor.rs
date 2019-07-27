use std::io::{Cursor, Write};
use std::str;

use super::output_test_helpers::*;


#[test]
fn nothing() {
    let buffer = render("line1 1234\nline2 23233");
    assert_output(&buffer, vec!["line1", "line2"])
}


pub fn render(text: &str) -> OutputBuffer {
    let mut buffer = Cursor::new(Vec::new());
    buffer.write(text.as_bytes()).unwrap();
    buffer
}
