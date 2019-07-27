use std::io::Cursor;
use std::str;

pub type OutputBuffer = Cursor<Vec<u8>>;
type Output<'a> = Vec<&'a str>;


pub fn assert_output(buffer: &OutputBuffer, expected_output: Output) {
    let rendered_output = output_from(buffer);
    assert_eq!(rendered_output.len(), expected_output.len(), "Number of lines in output not as expected");

    let line_length = expected_output[0].len();
    let actual_lines: Vec<String> = rendered_output
        .iter()
        .map(|s| s[0..line_length].to_string())
        .collect();

    assert_eq!(actual_lines, expected_output);
}

fn output_from(buffer: &OutputBuffer) -> Output {
    let actual_string = str::from_utf8(buffer.get_ref()).unwrap();
    actual_string.split("\n")
        .filter(|s| *s!="")
        .collect::<Output>()
        .to_vec()
}
