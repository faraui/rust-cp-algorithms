use std::io::{stdin, stdout, Write};
use std::collections::{VecDeque, BTreeMap};

#[derive(Clone, Debug)]
struct Dsu {
    p: Vec<usize>,
    s: Vec<usize>,
    n: usize,
    res: usize,
    q: Vec<Vec<(
        ((usize, usize), (usize, usize)),
        usize,
    )>>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let mut q = Vec::new();
        q.push(Vec::new());
        let mut p = vec![0; n];
        let mut s = vec![1; n];
        let res = n;
        for i in 0..n {
            p[i] = i;
        }
        Dsu { p, s, n, res, q }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        }
        self.p[v] = self.get(self.p[v]);
        self.p[v]
    }

    fn merge(&mut self, a: usize, b: usize) {
        let mut a = self.get(a);
        let mut b = self.get(b);
        if a == b {
            return;
        }
        if self.s[a] > self.s[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.q.last_mut().unwrap().push((((a, self.p[a]), (b, self.s[b])), self.res));
        self.res -= 1;
        self.p[a] = b;
        self.s[b] += self.s[a];
    }

    fn rollback(&mut self) {
        while !self.q.last().unwrap().is_empty() {
            let (
                ((a, pa), (b, sb)),
                res,
            ) = self.q.last_mut().unwrap().pop().unwrap();
            self.p[a] = pa;
            self.s[b] = sb;
            self.res = res;
        }
        self.q.pop();
    }

    fn save(&mut self) {
        self.q.push(Vec::new());
    }
}

fn upd(
    v: usize,
    ql: usize,
    qr: usize,
    l: usize,
    r: usize,
    x: (usize, usize),
    t: &mut Vec<Vec<(usize, usize)>>,
) {
    if r < ql || qr < l {
        return;
    }
    if ql <= l && r <= qr {
        t[v].push(x);
        return;
    }
    let m = (r + l) / 2;
    upd(v * 2, ql, qr, l, m, x, t);
    upd(v * 2 + 1, ql, qr, m + 1, r, x, t);
}

fn dfs(
    v: usize,
    l: usize,
    r: usize,
    d: &mut Dsu,
    t: &Vec<Vec<(usize, usize)>>,
    leaf_t: &Vec<bool>,
    res: &mut Vec<usize>,
) {
    let m = (r + l) / 2;
    d.save();
    for i in t[v].iter() {
        d.merge(i.0, i.1);
    }
    if l == r {
        if leaf_t[l] {
            res.push(d.res);
        }
        d.rollback();
        return;
    }
    dfs(v * 2, l, m, d, t, leaf_t, res);
    dfs(v * 2 + 1, m + 1, r, d, t, leaf_t, res);
    d.rollback();
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut res = Vec::new();
    let mut t = vec![Vec::new(); 4 * (m + 1)];
    let mut leaf_t = vec![false; m + 1];
    let mut mp = BTreeMap::new();
    let mut qs = VecDeque::new();

    let mut d = Dsu::new(n);

    for _ in 0..m {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let type_char = iter.next().unwrap().chars().next().unwrap();
        if type_char == '+' {
            let a: usize = iter.next().unwrap().parse().unwrap() - 1;
            let b: usize = iter.next().unwrap().parse().unwrap() - 1;
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            mp.insert((a, b), _);
        } else if type_char == '-' {
            let a: usize = iter.next().unwrap().parse().unwrap() - 1;
            let b: usize = iter.next().unwrap().parse().unwrap() - 1;
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            let i = mp.get(&(a, b)).unwrap();
            upd(1, *i, _, 0, m, (a, b), &mut t);
            mp.insert((a, b), usize::MAX);
        } else {
            qs.push_back(_);
        }
    }

    for i in qs.iter() {
        leaf_t[*i] = true;
    }

    for (i, &j) in mp.iter() {
        if j != usize::MAX {
            upd(1, j, m, 0, m, *i, &mut t);
        }
    }

    dfs(1, 0, m, &mut d, &t, &leaf_t, &mut res);

    for i in res {
        println!("{}", i);
    }
}
