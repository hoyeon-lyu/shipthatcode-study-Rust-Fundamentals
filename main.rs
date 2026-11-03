use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    // TODO: choose between FizzBuzz, Fizz, Buzz and the number itself.
    let mut state: &str = "";
    if n % 3 == 0 {
        if n % 5 == 0 {
            state = "FizzBuzz";
        } else {
            state = "Fizz";
        }
    } else if n % 5 == 0 {
        state = "Buzz";
    } else {
        println!("{}", n);
    }
    if state != "" {
        println!("{}", state);
    }
}
