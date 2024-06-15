use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
            j = p[j - 1];
        }
        if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() {
            j += 1;
        }
        p[i] = j;
    }
    println!("{:?}", p);
}
