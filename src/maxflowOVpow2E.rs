use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

type d1 = Vec<usize>;
type d2 = Vec<d1>;
type ll = usize;

fn build_net(d: &mut d1, g: &d2, s: usize, n: usize) {
  let mut q = VecDeque::new();
  d.fill(-1);
  d[s] = 0;
  q.push_back(s);
  while !q.is_empty() {
    let v = q.pop_front().unwrap();
    for i in 0..n {
      if d[i] == -1 && g[v][i] > 0 {
        d[i] = d[v] + 1;
        q.push_back(i);
      }
    }
  }
}

fn push(g: &mut d2, d: &d1, dd: &mut d1, v: usize, flow: ll, e: usize, n: usize) -> ll {
  if v == e {
    return flow;
  }
  while dd[v] < n {
    let to = dd[v];
    if d[to] == d[v] + 1 && g[v][to] > 0 {
      let addflow = push(g, d, dd, to, std::cmp::min(flow, g[v][to]), e, n);
      if addflow > 0 {
        g[v][to] -= addflow;
        g[to][v] += addflow;
        return addflow;
      }
    }
    dd[v] += 1;
  }
  0
}

fn main() {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  let mut split = input.split_whitespace();
  let n: ll = split.next().unwrap().parse().unwrap();
  let m: ll = split.next().unwrap().parse().unwrap();

  let mut g = d2::with_capacity(n);
  for _ in 0..n {
    g.push(d1::with_capacity(n));
    for _ in 0..n {
      g.last_mut().unwrap().push(0);
    }
  }

  for _ in 0..m {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let a: ll = split.next().unwrap().parse().unwrap();
    let b: ll = split.next().unwrap().parse().unwrap();
    let c: ll = split.next().unwrap().parse().unwrap();
    g[a - 1][b - 1] = c;
  }

  let s = 0;
  let e = n - 1;
  let mut flow = 0;
  let mut d = d1::with_capacity(n);
  let mut dd = d1::with_capacity(n);

  loop {
    build_net(&mut d, &g, s, n);
    if d[e] == -1 {
      break;
    }
    dd.fill(0);
    loop {
      let addflow = push(&mut g, &d, &mut dd, s, 1_000_000_000_000_000, e, n);
      if addflow == 0 {
        break;
      } else {
        flow += addflow;
      }
    }
  }
  print!("{}", flow);
  stdout().flush().unwrap();
}
