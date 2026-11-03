use std::io::{self, BufRead};

fn square(n: i64) -> i64 {
    // TODO: return n multiplied by itself.
    n * n
}

fn sum_of_squares(a: i64, b: i64) -> i64 {
    // TODO: return the square of a plus the square of b, using square().
    square(a) + square(b)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let b: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    println!("{}", square(a));
    println!("{}", square(b));
    println!("{}", sum_of_squares(a, b));
}
