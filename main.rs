use std::io::{self, BufRead};

enum Shape {
    Square(i64),
    Rect(i64, i64),
    Tri(i64, i64),
}

// Return the area of one shape. Square(side), Rect(width, height),
// Tri(base, height) for a right triangle, rounded down.
fn square(value: i64) -> i64{
    return value * value;
}

fn area(shape: &Shape) -> i64 {
    // TODO: match on shape and handle every variant.
    match shape {
        Shape::Square(side) => square(*side),           //square()함수가 인자로 i64를 받기 때문에 *로 읽을 수 있게 해줘야함.
        Shape::Rect(width, height) => width * height,   //&i64와 &i64의 곱은 i64를 생성함.
        Shape::Tri(width, height) => width * height / 2,
        _ => 0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: i64 = parts[1].parse().unwrap();
        let shape = match parts[0] {
            "square" => Shape::Square(a),
            "rect" => Shape::Rect(a, parts[2].parse().unwrap()),
            _ => Shape::Tri(a, parts[2].parse().unwrap()),
        };
        shapes.push(shape);
    }
    let mut total: i64 = 0;
    for s in &shapes {
        let x = area(s);
        println!("{}", x);
        total += x;
    }
    println!("total: {}", total);
}
