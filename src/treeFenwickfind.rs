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

    fn find(&self, s: usize) -> usize {
        let mut cnt = 0;
        let mut s = s;

        for i in (0..=31).rev() {
            if cnt + (1 << i) < self.t.len() && self.t[cnt + (1 << i)] < s {
                cnt += 1 << i;
                s -= self.t[cnt];
            }
        }

        cnt
    }
}
