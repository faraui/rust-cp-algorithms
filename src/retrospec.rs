// https://ru.wikipedia.org/wiki/%D0%A0%D0%B5%D1%82%D1%80%D0%BE%D1%81%D0%BF%D0%B5%D0%BA%D1%82%D0%B8%D0%B2%D0%BD%D1%8B%D0%B9_%D0%B0%D0%BD%D0%B0%D0%BB%D0%B8%D0%B7

use std::io::{stdin, stdout, Write};
use std::collections::{VecDeque, Vec};

fn main() {
    let mut n: usize = 0;
    let mut m: usize = 0;
    stdin().read_line(&mut String::new()).unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    n = split.next().unwrap().parse().unwrap();
    m = split.next().unwrap().parse().unwrap();

    let mut gr: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut deg: Vec<usize> = vec![0; n];
    let mut d: Vec<i32> = vec![-1; n];

    for _ in 0..m {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let a: usize = split.next().unwrap().parse().unwrap() - 1;
        let b: usize = split.next().unwrap().parse().unwrap() - 1;
        gr[b].push(a);
        deg[a] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            q.push_back(i);
            d[i] = 0;
        }
    }

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for i in gr[v].iter() {
            deg[*i] -= 1;
            if d[*i] != -1 {
                continue;
            }
            if d[v] == 0 {
                d[*i] = 1;
                q.push_back(*i);
            } else if deg[*i] == 0 {
                d[*i] = 0;
                q.push_back(*i);
            }
        }
    }

    for i in 0..n {
        let output = if d[i] == 0 {
            "WIN"
        } else if d[i] == 1 {
            "LOSE"
        } else {
            "DRAW"
        };
        let mut stdout = stdout();
        stdout.write_all(output.as_bytes()).unwrap();
        stdout.write_all(b"n").unwrap();
    }
}
