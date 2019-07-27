use std::io::{Cursor, Write};
use std::str;

type OutputBuffer = Cursor<Vec<u8>>;
type Output<'a> = Vec<&'a str>;

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


pub fn assert_output(buffer: &OutputBuffer, expected_output: Output) {
    let rendered_output = output_from(buffer);
    assert_eq!(rendered_output.len(), expected_output.len(), "Number of rendered lines not as expected");

    let line_length = expected_output[0].len();
    let actual_lines: Vec<String> = rendered_output
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(actual_lines, expected_output);
}

fn output_from(buffer: &OutputBuffer) -> Output {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    let lines: Vec<&str> = actual_string.split("\n").collect();
    lines.to_vec()
}
