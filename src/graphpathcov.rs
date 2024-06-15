use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
struct D2<T> {
    data: Vec<Vec<T>>,
}

impl<T> D2<T> {
    fn new(n: usize) -> Self {
        D2 {
            data: vec![Vec::new(); n],
        }
    }
}

#[derive(Debug, Clone)]
struct D1<T> {
    data: Vec<T>,
}

impl<T> D1<T> {
    fn new(n: usize, val: T) -> Self {
        D1 {
            data: vec![val; n],
        }
    }
}

fn dfs(v: usize, g: &D2<usize>, mt: &mut D1<isize>, seen: &mut D1<usize>, color: usize) -> bool {
    if seen[v] == color {
        return false;
    }
    seen[v] = color;
    for &i in &g.data[v] {
        if mt[i] == -1 {
            mt[i] = v as isize;
            return true;
        }
    }
    for &i in &g.data[v] {
        if dfs(mt[i] as usize, g, mt, seen, color) {
            mt[i] = v as isize;
            return true;
        }
    }
    false
}

fn get(v: usize, p: &mut D1<usize>) -> usize {
    if p[v] == v {
        return v;
    }
    p[v] = get(p[v], p);
    p[v]
}

fn merge(a: usize, b: usize, p: &mut D1<usize>, s: &mut D1<usize>, cnt: &mut usize) {
    let a = get(a, p);
    let b = get(b, p);
    if s[a] > s[b] {
        std::mem::swap(&mut a, &mut b);
    }
    p[a] = b;
    s[b] += s[a];
    *cnt -= 1;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut g = D2::new(n);
    let mut mt = D1::new(n, -1);
    let mut seen = D1::new(n, 0);
    let mut cnt = n;
    let mut color = 0;

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse().unwrap() - 1;
        g.data[b].push(a);
    }

    for i in 0..n {
        color += 1;
        dfs(i, &g, &mut mt, &mut seen, color);
    }

    let mut p = D1::new(n, 0);
    let mut s = D1::new(n, 1);
    for i in 0..n {
        if mt[i] != -1 {
            merge(i, mt[i] as usize, &mut p, &mut s, &mut cnt);
        }
    }

    println!("{}", cnt);
}
