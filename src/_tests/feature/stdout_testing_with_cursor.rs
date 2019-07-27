use std::io::{Cursor, Write};
use std::str;

type LineBuffer = Cursor<Vec<u8>>;
type Lines<'a> = Vec<&'a str>;

#[test]
fn nothing() {
    let buffer = render("line1 1234\nline2 23233");
    assert_lines_start_with(&buffer, vec!["line1", "line2"])
}


pub fn render(text: &str) -> LineBuffer {
    let mut buffer = Cursor::new(Vec::new());
    buffer.write(text.as_bytes()).unwrap();
    buffer
}


pub fn assert_lines_start_with(buffer: &LineBuffer, expected_lines: Lines) {
    let rendered_lines = lines_from(buffer);
    assert_eq!(rendered_lines.len(), expected_lines.len(), "Number of rendered lines not as expected");

    let line_length = expected_lines[0].len();
    let actual_lines: Vec<String> = rendered_lines
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(actual_lines, expected_lines);
}

fn lines_from(buffer: &LineBuffer) -> Lines {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    let lines: Vec<&str> = actual_string.split("\n").collect();
    lines.to_vec()
}
