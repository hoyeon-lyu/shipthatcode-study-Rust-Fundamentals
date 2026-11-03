use std::io::{self, BufRead};

// Count the vowels (a, e, i, o, u, in either case) in the borrowed text.
fn vowel_count(s: &str) -> usize {
    let mut counts:usize = 0;
    let l = s.to_lowercase();
    for t in l.chars() {
        let vowel = match t {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false
        };
        if vowel { counts += 1; }
    }
    counts
}

// Change the caller's String in place: make it uppercase, then make sure
// it ends with '!' (add one only if it does not already end in '!').
fn make_loud(s: &mut String) {
    *s = s.to_uppercase();
    let p: &str = "!";
    if !s.ends_with("!") { *s = s.to_owned() + p }
}

fn main() {
    let stdin = io::stdin();
    let mut line = stdin.lock().lines().next().unwrap().unwrap();
    let before = vowel_count(&line);
    make_loud(&mut line);
    println!("{}", before);
    println!("{}", line);
    println!("{}", vowel_count(&line));
}
