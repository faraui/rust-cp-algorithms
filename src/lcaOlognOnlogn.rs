use std::io::{stdin, stdout, BufWriter, Write};

fn la(v: usize, d: usize, p: &Vec<Vec<usize>>, k: usize) -> usize {
    for i in (0..k).rev() {
        if d & (1 << i) != 0 {
            v = p[v][i];
        }
    }
    v
}

fn lca(a: usize, b: usize, p: &Vec<Vec<usize>>, h: &Vec<i32>) -> usize {
    let k = p[0].len();
    if h[a] > h[b] {
        let temp = a;
        a = b;
        b = temp;
    }
    let b = la(b, h[b] as usize - h[a] as usize, p, k);

    for i in (0..k).rev() {
        if p[a][i] != p[b][i] {
            a = p[a][i];
            b = p[b][i];
        }
    }

    if a != b {
        a = p[a][0];
    }
    a
}

fn dfs(v: usize, p: &mut Vec<Vec<usize>>, h: &mut Vec<i32>) {
    if h[v] == -1 {
        dfs(p[v][0], p, h);
    } else {
        return;
    }
    h[v] = h[p[v][0]] + 1;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let k = (n as f64).log2().ceil() as usize + 1;

    let mut p = vec![vec![0; k]; n];
    let mut h = vec![-1; n];
    p[0][0] = 0;
    h[0] = 0;

    for i in 1..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        p[i][0] = input.trim().parse().unwrap();
    }

    for i in 0..n {
        if h[i] == -1 {
            dfs(i, &mut p, &mut h);
        }
    }

    for i in 0..k {
        for j in 0..n {
            p[j][i + 1] = p[p[j][i]][i];
        }
    }

    let mut output = BufWriter::new(stdout());
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let a: usize = split.next().unwrap().parse().unwrap();
    let b: usize = split.next().unwrap().parse().unwrap();

    let result = lca(a, b, &p, &h);
    writeln!(output, "{}", result).unwrap();
}
