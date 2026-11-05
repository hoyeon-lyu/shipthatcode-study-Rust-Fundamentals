use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut largest: i64 = i64::MIN;
    for num in nums {
        if num > largest {
            largest = num;
        }
    }

    // TODO: walk `nums`, keep the largest value seen so far,
    // and print it once after the loop.
    println!("{}", largest); // placeholder: prints the count, not the answer; replace it
}
