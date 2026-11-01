use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();

    // TODO: take a trimmed &str view of `line`, then print three lines:
    //   1. that text in uppercase
    //   2. how many characters it has
    //   3. how many bytes it has

    let sen: &str = line.trim();
    let sen_upper: String = sen.to_uppercase();
    let chars = sen.chars().count();
    let bytes = sen.len();
    
    println!("{}", sen_upper);
    println!("{}", chars);
    println!("{}", bytes);
}
