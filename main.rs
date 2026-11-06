use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // TODO: build an iterator pipeline over nums that keeps only the
    // even values, squares each one, and adds them up.
    let mut total: i64 = 0;

    nums = nums.iter()
        .filter(|s| **s % 2 == 0)
        .map(|s| s * s)
        .collect();
    
    total = nums.iter().sum();
    println!("{}", total);
}
