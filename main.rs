use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first = lines.next().unwrap().unwrap();
    let second = lines.next().unwrap().unwrap();

    // TODO: shadow `first` and `second` as i64 numbers (the text may have spaces around it).
    // TODO: keep a mutable running total starting at 0; add each number and print the total after each addition.
    let _ = (&first, &second);

    let first:i64 = first.trim().parse().unwrap();
    let second:i64 = second.trim().parse().unwrap();
    
    let mut total = 0;
    total += first;
    println!("{}",total);

    total += second;
    println!("{}",total);
}
