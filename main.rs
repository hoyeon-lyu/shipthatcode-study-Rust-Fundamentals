use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();

    let mut counts: HashMap<&str, i64> = HashMap::new();
    // TODO 1: count how many times each word appears, using counts.entry(...).

    for word in &words {
        *counts.entry(word).or_insert(0) += 1_i64;
    }

    let mut best_word = words[0];
    let mut best_count: i64 = 0_i64;
    // TODO 2: walk `words` in input order, look up each word's count,
    // and keep the word with the highest count (the earliest one wins a tie).

    for word in &words {
        let count_now = match counts.get(word) {
            Some(n) => n.clone(),
            _ => 0_i64
        };
        if count_now > best_count {
            best_word = word;
            best_count = count_now;
        }
    }

    println!("{} {}", best_word, best_count);
}
