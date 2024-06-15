// https://codeforces.com/problemset/problem/321/C?

use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;

fn sz(v: usize, p: usize, d: &mut Vec<usize>, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    d[v] = 1;
    for i in g[v].iter() {
        if *i != p && !seen[*i] {
            sz(*i, v, d, g, seen);
            d[v] += d[*i];
        }
    }
}

fn centroid(v: usize, p: usize, n: usize, d: &Vec<usize>, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    for i in g[v].iter() {
        if *i != p && !seen[*i] {
            if d[*i] > n / 2 {
                return centroid(*i, v, n, d, g, seen);
            }
        }
    }
    v
}

fn dfs(v: usize, n: usize, seen: &mut Vec<bool>, d: &mut Vec<usize>, g: &Vec<Vec<usize>>, cg: &mut Vec<Vec<usize>>) -> usize {
    sz(v, n, d, g, seen);
    let c = centroid(v, n, d[v], d, g, seen);
    seen[c] = true;
    for i in g[c].iter() {
        if !seen[*i] {
            cg[c].push(dfs(*i, n, seen, d, g, cg));
        }
    }
    c
}

fn dfs_color(v: usize, c: char, cg: &Vec<Vec<usize>>, colors: &mut Vec<char>) {
    colors[v] = c;
    for i in cg[v].iter() {
        dfs_color(*i, (c as u8 + 1) as char, cg, colors);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut seen = vec![false; n];
    let mut d = vec![0; n];
    let mut g = vec![Vec::new(); n];
    let mut cg = vec![Vec::new(); n];
    let mut colors = vec![' '; n];

    for _ in 0..(n - 1) {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap() - 1;
        let b: usize = iter.next().unwrap().parse().unwrap() - 1;
        g[a].push(b);
        g[b].push(a);
    }

    let root = dfs(0, n, &mut seen, &mut d, &mut g, &mut cg);
    dfs_color(root, 'A', &cg, &mut colors);

    for i in 0..n {
        print!("{} ", colors[i]);
    }
}
