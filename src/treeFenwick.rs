use std::vec::Vec;

struct FenwickTree {
    t: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { t: vec![0; n + 1] }
    }

    fn upd(&mut self, p: usize, x: usize) {
        let mut p = p;
        while p < self.t.len() {
            self.t[p] += x;
            p += p & (!p + 1);
        }
    }

    fn get(&self, p: usize) -> usize {
        let mut p = p;
        let mut res = 0;
        while p > 0 {
            res += self.t[p];
            p -= p & (!p + 1);
        }
        res
    }
}
