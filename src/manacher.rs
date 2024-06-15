use std::io::{stdin, stdout, Write};

fn manacher() {
    let mut s1 = String::new();
    stdin().read_line(&mut s1).unwrap();
    let mut s = String::from("#");
    for c in s1.trim().chars() {
        s.push(c);
        s.push('#');
    }
    let n = s.len();
    let mut p = vec![0; n];
    let mut l = 0;
    let mut r = -1;
    for i in 0..n {
        if i <= r {
            p[i] = std::cmp::min(r - i, p[r + l - i]);
        }
        while i + p[i] < n && i - p[i] >= 0 && s.chars().nth(i + p[i]).unwrap() == s.chars().nth(i - p[i]).unwrap() {
            p[i] += 1;
        }
        p[i] -= 1;
        if i + p[i] > r {
            l = i - p[i];
            r = i + p[i];
        }
    }
    let mut res = 0;
    for i in 0..n {
        res += p[i];
        if i % 2 == 1 {
            res += 1;
        }
    }
    println!("{}", res / 2);
}
