use std::io::{self, Read};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_sq(&self, other: &Point) -> i32 {
        // TODO: return the squared distance between `self` and `other`,
        // using the fields of both points.
        return square(other.x - self.x) + square(other.y - self.y)
    }
}

fn square(value: i32) -> i32 {
    return value * value;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a = Point { x: v[0], y: v[1] };
    let b = Point { x: v[2], y: v[3] };
    println!("{}", a.distance_sq(&b));
}
