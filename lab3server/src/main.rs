use std::io::Write;

fn main() {
    writeln!(std::io::stdout().lock(), "Hello, world!").expect("Failed to write to stderr");
}
