use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let w: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let h: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let k: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    // TODO: compute the area, each person's whole share, and the leftover,
    // then print those three numbers instead of the zeros below.
    let _ = (w, h, k);

    let area:i64 = w * h;
    let share:i64 = area / k;
    let l_over:i64 = area % k;
    
    println!("{}", area);
    println!("{}", share);
    println!("{}", l_over);
}
