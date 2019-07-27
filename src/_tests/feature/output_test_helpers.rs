use std::io::Cursor;
use std::str;
use crate::game::Game;

pub type OutputBuffer = Cursor<Vec<u8>>;
type OutputLines<'a> = Vec<&'a str>;


pub fn verify_frame(game: &Game, expected_output: OutputLines) {
    let mut cursor = Cursor::new(Vec::new());
    game.render(&mut cursor).unwrap();
    assert_output_column(&cursor, expected_output)
}

pub fn assert_output_column(buffer: &OutputBuffer, expected_output: OutputLines) {
    let line_length = expected_output[0].len();
    let output_trimmed: String = output_lines_from(buffer)[0..expected_output.len()]
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect::<Vec<String>>()
        .join("\n");

    assert_eq!(output_trimmed, expected_output.join("\n"));
}


fn output_lines_from(buffer: &OutputBuffer) -> OutputLines {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    actual_string.split("\n")
        .filter(|s| *s != "")
        .collect::<OutputLines>()
        .to_vec()
}