use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;

fn la(v: usize, d: usize, p: &Vec<usize>, h: &Vec<usize>, jmp: &Vec<usize>) -> usize {
    let mut d = d;
    let mut v = v;
    while d > 0 {
        if p[v] == v {
            return v;
        }
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

fn lca(a: usize, b: usize, p: &Vec<usize>, h: &Vec<usize>, jmp: &Vec<usize>) -> usize {
    let mut a = a;
    let mut b = b;
    if h[a] < h[b] {
        std::mem::swap(&mut a, &mut b);
    }
    a = la(a, h[a] - h[b], p, h, jmp);
    while a != b {
        if jmp[a] != jmp[b] {
            a = jmp[a];
            b = jmp[b];
        } else {
            a = p[a];
            b = p[b];
        }
    }
    a
}


fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut p = vec![0; n];
    let mut h = vec![0; n];
    let mut jmp = vec![0; n];
    let mut g = vec![Vec::new(); n];

    p[0] = 0;
    jmp[0] = 0;
    h[0] = 0;

    for i in 1..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        p[i] = input.trim().parse().unwrap();
        g[p[i]].push(i);
    }

    let mut q = VecDeque::new();
    q.push_back(0);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        h[v] = h[p[v]] + 1;

        if h[p[v]] - h[jmp[p[v]]] == h[jmp[p[v]]] - h[jmp[jmp[p[v]]]] {
            jmp[v] = jmp[jmp[p[v]]];
        } else {
            jmp[v] = p[v];
        }

        for i in g[v].iter() {
            q.push_back(*i);
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();

    println!("{}", lca(a, b, &p, &h, &jmp));
}
