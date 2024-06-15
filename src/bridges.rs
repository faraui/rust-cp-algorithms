use std::io::{stdin, stdout, Write};
use std::vec::Vec;

type d2 = Vec<Vec<usize>>;
type d1 = Vec<usize>;
type ll = usize;

fn dfs(v: ll, p: ll, g: &d2, seen: &mut d1, tin: &mut d1, d: &mut d1, mark: &mut d2, timer: &mut ll) {
    seen[v] = 1;
    d[v] = tin[v] = *timer;
    *timer += 1;
    for i in g[v].iter() {
        if *i != p {
            if seen[*i] == 0 {
                dfs(*i, v, g, seen, tin, d, mark, timer);
                d[v] = std::cmp::min(d[v], d[*i]);
                if d[*i] > tin[v] {
                    mark[v][*i] = 1;
                }
            } else {
                d[v] = std::cmp::min(d[v], tin[*i]);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: ll = parts.next().unwrap().parse().unwrap();
    let m: ll = parts.next().unwrap().parse().unwrap();

    let mut g: d2 = vec![d1::new(); n];
    let mut seen: d1 = vec![0; n];
    let mut tin: d1 = vec![0; n];
    let mut d: d1 = vec![0; n];
    let mut mark: d2 = vec![vec![0; n]; n];

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let a: ll = parts.next().unwrap().parse().unwrap() - 1;
        let b: ll = parts.next().unwrap().parse().unwrap() - 1;
        g[a].push(b);
        g[b].push(a);
    }

    let mut timer: ll = 0;
    for i in 0..n {
        if seen[i] == 0 {
            dfs(i, std::usize::MAX, &g, &mut seen, &mut tin, &mut d, &mut mark, &mut timer);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if mark[i][j] == 1 {
                let output = format!("{} {}n", i + 1, j + 1);
                stdout().write_all(output.as_bytes()).unwrap();
            }
        }
    }
}
