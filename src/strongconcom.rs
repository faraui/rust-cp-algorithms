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
    fn assign(&mut self, n: usize, val: D1<T>) {
        self.data = vec![val; n];
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
    fn push_back(&mut self, val: T) {
        self.data.push(val);
    }
    fn assign(&mut self, n: usize, val: T) {
        self.data = vec![val; n];
    }
}

fn dfs1(v: usize, g: &D2<usize>, seen: &mut D1<usize>, ts: &mut D1<usize>) {
    seen[v] = 1;
    for &i in &g.data[v] {
        if seen[i] == 0 {
            dfs1(i, g, seen, ts);
        }
    }
    ts.push_back(v);
}

fn dfs2(v: usize, c: usize, gr: &D2<usize>, seen: &mut D1<usize>) {
    seen[v] = c;
    for &i in &gr.data[v] {
        if seen[i] == 0 {
            dfs2(i, c, gr, seen);
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut g = D2::new(n);
    let mut gr = D2::new(n);
    let mut seen = D1::new(n, 0);
    let mut ts = D1::new(0, 0);

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse().unwrap() - 1;
        g.data[a].push(b);
        gr.data[b].push(a);
    }

    for i in 0..n {
        if seen[i] == 0 {
            dfs1(i, &g, &mut seen, &mut ts);
        }
    }

    ts.data.reverse();
    seen.assign(n, 0);
    let mut cnt = 0;
    for &i in &ts.data {
        if seen[i] == 0 {
            cnt += 1;
            dfs2(i, cnt, &gr, &mut seen);
        }
    }

    println!("{}", cnt);
    // print(used);
}
