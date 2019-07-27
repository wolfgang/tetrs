use console::Term;

fn main() -> std::io::Result<()> {
    let stdout = Term::stdout();
    stdout.write_line("Hello, world!")
}
