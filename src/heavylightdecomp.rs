use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

struct HLD {
    g: Vec<Vec<usize>>,
    t: Vec<i32>,
    d: Vec<usize>,
    p: Vec<usize>,
    head: Vec<usize>,
    tin: Vec<usize>,
    tout: Vec<usize>,
}

impl HLD {
    fn new(n: usize) -> Self {
        HLD {
            g: vec![Vec::new(); n],
            t: vec![0; n],
            d: vec![0; n],
            p: vec![0; n],
            head: vec![0; n],
            tin: vec![0; n],
            tout: vec![0; n],
        }
    }

    fn sz(&mut self, v: usize, prev: usize) {
        self.d[v] = 1;
        self.p[v] = prev;
        for i in 0..self.g[v].len() {
            if self.g[v][i] == prev {
                continue;
            }
            self.sz(self.g[v][i], v);
            self.d[v] += self.d[self.g[v][i]];
            if self.d[self.g[v][i]] > self.d[self.g[v][0]] {
                self.g[v].swap(i, 0);
            }
        }
    }

    fn dfs(&mut self, v: usize, prev: usize, timer: &mut usize) {
        self.tin[v] = *timer;
        *timer += 1;
        for i in 0..self.g[v].len() {
            if self.g[v][i] == prev {
                continue;
            }
            if self.g[v][i] == self.g[v][0] {
                self.head[self.g[v][i]] = self.head[v];
            } else {
                self.head[self.g[v][i]] = self.g[v][i];
            }
            self.dfs(self.g[v][i], v, timer);
        }
        self.tout[v] = *timer;
    }

    fn upd(&mut self, p: usize, x: i32) {
        for i in self.tin[p]..self.tout[p] {
            self.t[i] += x;
        }
    }

    fn get(&self, p: usize) -> i32 {
        let mut res = 0;
        for i in (self.tin[p]..self.tout[p]).rev() {
            res += self.t[i];
        }
        res
    }

    fn parent(&self, a: usize, b: usize) -> bool {
        self.tin[a] <= self.tin[b] && self.tin[b] < self.tout[a]
    }

    fn up(&mut self, a: &mut usize, b: usize, res: &mut i32) {
        while !self.parent(self.head[*a], b) {
            *res += self.get(self.tin[*a]) - self.get(self.tin[self.head[*a]] - 1);
            *a = self.p[self.head[*a]];
        }
    }

    fn get(&mut self, a: usize, b: usize) -> i32 {
        let mut res = 0;
        self.up(&mut a, b, &mut res);
        self.up(&mut b, a, &mut res);
        if !self.parent(a, b) {
            std::mem::swap(&mut a, &mut b);
        }
        res += self.get(self.tin[b]) - self.get(self.tin[a] - 1);
        res
    }
}

fn main() {
    let mut n: usize = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut g: Vec<Vec<usize>> = Vec::new();
    let mut t: Vec<i32> = Vec::new();
    let mut d: Vec<usize> = Vec::new();
    let mut p: Vec<usize> = Vec::new();
    let mut head: Vec<usize> = Vec::new();
    let mut tin: Vec<usize> = Vec::new();
    let mut tout: Vec<usize> = Vec::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    n = input.trim().parse().unwrap();
    input.clear();
    a.resize(n, 0);
    for i in 0..n {
        stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().unwrap();
        input.clear();
    }
    g.resize(n, Vec::new());
    t.resize(n + 1, 0);
    d.resize(n, 0);
    p.resize(n, 0);
    head.resize(n, 0);
    tin.resize(n, 0);
    tout.resize(n, 0);

    for i in 1..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let v: usize = parts.next().unwrap().parse().unwrap() - 1;
        let u: usize = parts.next().unwrap().parse().unwrap() - 1;
        input.clear();
        g[v].push(u);
        g[u].push(v);
    }

    let mut hld = HLD {
        g,
        t,
        d,
        p,
        head,
        tin,
        tout,
    };

    hld.sz(0, 0);
    let mut timer = 1;
    hld.dfs(0, 0, &mut timer);

    for i in 0..n {
        hld.upd(hld.tin[i], a[i]);
    }

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let q = input.trim().chars().next().unwrap();
        input.clear();
        if q == '!' {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let v: usize = parts.next().unwrap().parse().unwrap() - 1;
            let x: i32 = parts.next().unwrap().parse().unwrap();
            input.clear();
            hld.upd(hld.tin[v], -a[v]);
            a[v] = x;
            hld.upd(hld.tin[v], a[v]);
        } else {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let a: usize = parts.next().unwrap().parse().unwrap() - 1;
            let b: usize = parts.next().unwrap().parse().unwrap() - 1;
            input.clear();
            println!("{}", hld.get(a, b));
        }
        stdout().flush().unwrap();
    }
}
