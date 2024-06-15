use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

type d2 = Vec<Vec<usize>>;
type d1 = Vec<usize>;
type ll = usize;

fn push(from: usize, to: usize, e: &mut d1, g: &mut d2, flow: &mut usize, t: usize) {
  let addflow = std::cmp::min(e[from], g[from][to]);
  if to == t {
    *flow += addflow;
  }
  if e[to] == 0 && to != t {
    q.push_back(to);
  }
  e[from] -= addflow;
  e[to] += addflow;
  g[from][to] -= addflow;
  g[to][from] += addflow;
}

fn lift(v: usize, g: &d2, h: &mut d1) {
  let mut lvl = std::usize::MAX;
  for i in 0..g.len() {
    if g[v][i] > 0 {
      lvl = std::cmp::min(lvl, h[i]);
    }
  }
  if lvl != std::usize::MAX {
    h[v] = lvl + 1;
  }
}

fn discharge(v: usize, e: &mut d1, g: &mut d2, h: &mut d1) {
  while e[v] > 0 {
    for i in 0..g.len() {
      if g[v][i] > 0 && h[v] == h[i] + 1 {
        push(v, i, e, g, &mut flow, t);
      }
    }
    lift(v, g, h);
  }
}

fn main() {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  let mut parts = input.trim().split_whitespace();
  let n: usize = parts.next().unwrap().parse().unwrap();
  let m: usize = parts.next().unwrap().parse().unwrap();

  let mut g = vec![vec![0; n]; n];
  let mut e = vec![0; n];
  let mut h = vec![0; n];
  let mut flow = 0;
  let mut q = VecDeque::new();

  for _ in 0..m {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let a: usize = parts.next().unwrap().parse().unwrap();
    let b: usize = parts.next().unwrap().parse().unwrap();
    let c: usize = parts.next().unwrap().parse().unwrap();
    g[a - 1][b - 1] = c;
  }

  let s = 0;
  let t = n - 1;
  e[s] = std::usize::MAX;
  h[s] = n;

  for i in 0..n {
    push(s, i, &mut e, &mut g, &mut flow, t);
  }

  while !q.is_empty() {
    let v = q.pop_front().unwrap();
    discharge(v, &mut e, &mut g, &mut h);
  }

  print!("{}", flow);
  stdout().flush().unwrap();
}
