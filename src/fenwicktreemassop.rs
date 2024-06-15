use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
struct Node {
    f: usize,
    s: usize,
}

struct FenwickTree {
    t: Vec<Node>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            t: vec![Node { f: 0, s: 0 }; n + 1],
        }
    }

    fn upd(&mut self, r: usize, x: usize) {
        let mut r = r;
        if r == 0 {
            return;
        }
        while r > 0 {
            self.t[r].s += x;
            r -= r & (!r + 1);
        }
        let mut r = r + (r & (!r + 1));
        while r < self.t.len() {
            self.t[r].f += x * (r - (r & (!r + 1)));
            r += r & (!r + 1);
        }
    }

    fn get(&self, r: usize) -> usize {
        if r == 0 {
            return 0;
        }
        let mut r = r;
        let mut res = 0;
        while r > 0 {
            res += self.t[r].f + self.t[r].s * (r & (!r + 1));
            r -= r & (!r + 1);
        }
        let mut r = r + (r & (!r + 1));
        while r < self.t.len() {
            res += self.t[r].s * (r - (r & (!r + 1)));
            r += r & (!r + 1);
        }
        res
    }
}
