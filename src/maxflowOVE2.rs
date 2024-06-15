use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut g = vec![vec![0; n]; n];

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse().unwrap() - 1;
        let c: usize = parts.next().unwrap().parse().unwrap();

        g[a][b] = c;
    }

    let s: usize = 0;
    let e: usize = n - 1;

    let mut flow: usize = 0;
    let mut p = vec![0; n];
    let mut seen = vec![0; n];
    let mut t: usize = 1;
    let mut q = VecDeque::new();

    loop {
        q.push_back(s);

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for i in 0..n {
                if seen[i] < t && g[v][i] > 0 {
                    seen[i] = t;
                    q.push_back(i);
                    p[i] = v;
                }
            }
        }

        if seen[e] < t {
            break;
        }

        let mut addflow: usize = usize::MAX;
        let mut v = e;
        while v != s {
            addflow = addflow.min(g[p[v]][v]);
            v = p[v];
        }

        v = e;
        while v != s {
            g[p[v]][v] -= addflow;
            g[v][p[v]] += addflow;
            v = p[v];
        }

        flow += addflow;
        t += 1;
    }

    let output = format!("{}n", flow);
    stdout().write_all(output.as_bytes()).unwrap();
}
