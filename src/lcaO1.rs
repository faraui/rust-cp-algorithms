use std::io::{stdin, stdout, BufWriter, Write};
use std::cmp::min;

fn dfs(
    v: usize,
    p: usize,
    lvl: usize,
    g: &mut Vec<Vec<usize>>,
    h: &mut Vec<usize>,
    tin: &mut Vec<usize>,
    way: &mut Vec<(usize, usize)>,
) {
    h[v] = lvl;
    tin[v] = way.len();
    way.push((h[v], v));
    for &i in &g[v] {
        if i != p {
            dfs(i, v, lvl + 1, g, h, tin, way);
            way.push((h[v], v));
        }
    }
}

fn get(l: usize, r: usize, st: &Vec<Vec<(usize, usize)>>) -> usize {
    let len = r - l + 1;
    let k = (len as f64).log2().ceil() as usize;
    if st[k][l].0 < st[k][r - (1 << k) + 1].0 {
        st[k][l].1
    } else {
        st[k][r - (1 << k) + 1].1
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();

    let mut g = vec![Vec::new(); n];
    let mut h = vec![0; n];
    let mut tin = vec![0; n];
    let mut way = Vec::new();
    let mut st = vec![vec![(0, 0); n]; (n as f64).log2().ceil() as usize + 1];

    for i in 1..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let x: usize = input.trim().parse().unwrap();
        g[i].push(x);
        g[x].push(i);
    }

    dfs(0, usize::MAX, 0, &mut g, &mut h, &mut tin, &mut way);

    for i in 0..way.len() {
        st[0][i] = way[i];
    }

    let k = (way.len() as f64).log2().ceil() as usize;

    for i in 1..=k {
        for j in 0..way.len() {
            if j + (1 << i) > way.len() {
                break;
            }
            if st[i - 1][j].0 < st[i - 1][j + (1 << (i - 1))].0 {
                st[i][j] = st[i - 1][j];
            } else {
                st[i][j] = st[i - 1][j + (1 << (i - 1))];
            }
        }
    }

    let mut output = BufWriter::new(stdout());
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let a: usize = split.next().unwrap().parse().unwrap();
    let b: usize = split.next().unwrap().parse().unwrap();

    let result = get(tin[a], tin[b], &st);
    writeln!(output, "{}", result).unwrap();
}
