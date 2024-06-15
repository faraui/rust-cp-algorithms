use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

type d2 = Vec<VecDeque<usize>>;
type d1 = Vec<usize>;
type ll = usize;

let mut g: d2;
let mut seen: d1;
let mut tin: d1;
let mut d: d1;
let mut cut: d1;
let mut timer: ll = 0;

fn dfs(v: ll, p: ll) {
    seen[v] = 1;
    d[v] = tin[v] = timer;
    timer += 1;
    let mut x: ll = 0;
    for i in g[v].iter() {
        if seen[*i] == 0 {
            dfs(*i, v);
            d[v] = std::cmp::min(d[v], d[*i]);
            if d[*i] >= tin[v] && p != std::usize::MAX {
                cut[v] = 1;
            }
            x += 1;
        } else {
            d[v] = std::cmp::min(d[v], tin[*i]);
        }
    }
    if p == std::usize::MAX && x > 1 {
        cut[v] = 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: ll = iter.next().unwrap().parse().unwrap();
    let m: ll = iter.next().unwrap().parse().unwrap();

    g = vec![VecDeque::new(); n];
    seen = vec![0; n];
    tin = vec![0; n];
    d = vec![0; n];
    cut = vec![0; n];

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let a: ll = iter.next().unwrap().parse().unwrap() - 1;
        let b: ll = iter.next().unwrap().parse().unwrap() - 1;
        g[a].push_back(b);
        g[b].push_back(a);
    }
    for i in 0..n {
        if seen[i] == 0 {
            dfs(i, std::usize::MAX);
        }
    }

    let mut count: ll = 0;
    for i in 0..n {
        if cut[i] == 1 {
            count += 1;
        }
    }
    print!("{}", count);
    stdout().flush().unwrap();
    println!();
    for i in 0..n {
        if cut[i] == 1 {
            print!("{} ", i + 1);
        }
    }
}
