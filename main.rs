use std::io;

fn main() {
    // Already done for you: read one line of input into `name` and drop the
    // Enter key from its end. A later lesson explains how this works.
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let name = line.trim_end_matches(|c| c == '\r' || c == '\n');

    // TODO: print the two lines described in the exercise.
    // Use a {} placeholder for name; do not type any name yourself.
    let _ = name;
}
