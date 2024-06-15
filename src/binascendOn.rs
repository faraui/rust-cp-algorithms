use std::collections::VecDeque;
use std::io::{stdin, stdout, BufWriter, Write};

fn get(v: usize, mut d: usize, p: &Vec<usize>, jmp: &Vec<usize>, h: &Vec<usize>) -> usize {
    while d != 0 && v != p[v] {
        if h[v] - h[jmp[v]] <= d {
            d -= h[v] - h[jmp[v]];
            v = jmp[v];
        } else {
            d -= 1;
            v = p[v];
        }
    }
    v
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();

    let mut p = vec![0; n];
    let mut h = vec![0; n];
    let mut jmp = vec![0; n];
    let mut g = vec![Vec::new(); n];

    p[0] = jmp[0] = h[0] = 0;

    for i in 1..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        p[i] = input.trim().parse().unwrap();
        g[p[i]].push(i);
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        h[v] = h[p[v]] + 1;

        if h[p[v]] - h[jmp[p[v]]] == h[jmp[p[v]]] - h[jmp[jmp[p[v]]]] {
            jmp[v] = jmp[jmp[p[v]]];
        } else {
            jmp[v] = p[v];
        }

        for &i in &g[v] {
            q.push_back(i);
        }
    }

    let mut output = BufWriter::new(stdout());
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let v: usize = split.next().unwrap().parse().unwrap();
    let dist: usize = split.next().unwrap().parse().unwrap();

    let result = get(v, dist, &p, &jmp, &h);
    writeln!(output, "{}", result).unwrap();
}
