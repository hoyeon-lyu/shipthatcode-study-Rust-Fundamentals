use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let name = lines.next().unwrap().unwrap().trim().to_string();
    let age: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let height: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // TODO: give each placeholder the format spec the badge needs.
    println!("[{:<10}]", name);
    println!("Age: {:>03}", age);
    println!("Height: {:.2} m", height);
}
