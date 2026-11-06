use std::io::{self, BufRead};

fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: turn `s` into an i32 and return it inside Ok.
    // Anything that is not a valid i32 must come back as
    // Err with the message "not a number".
    match s.trim().parse::<i32>() {
        Result::Ok(p) => Ok(p),
        Result::Err(_) => Err(String::from("not a number"))
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match parse_int(&line) {
            Ok(n) => println!("ok: {}", n),
            Err(e) => println!("error: {}", e),
        }
    }
}
