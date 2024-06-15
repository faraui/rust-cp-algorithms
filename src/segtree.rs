use std::vec::Vec;

struct SegmentTree {
    t: Vec<usize>,
    a: Vec<usize>,
}

impl SegmentTree {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        let mut t = vec![0; 4 * n];
        let mut st = SegmentTree { t, a };
        st.build(1, 0, n - 1);
        st
    }

    fn build(&mut self, v: usize, l: usize, r: usize) {
        if l == r {
            self.t[v] = self.a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * v, l, m);
        self.build(2 * v + 1, m + 1, r);
        self.t[v] = self.t[2 * v] + self.t[2 * v + 1];
    }

    fn get(&self, v: usize, ql: usize, qr: usize, l: usize, r: usize) -> usize {
        if ql > r || l > qr {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.t[v];
        }
        let m = (l + r) / 2;
        self.get(2 * v, ql, qr, l, m) + self.get(2 * v + 1, ql, qr, m + 1, r)
    }

    fn upd(&mut self, v: usize, l: usize, r: usize, p: usize, x: usize) {
        if l == r {
            self.t[v] = x;
            return;
        }
        let m = (l + r) / 2;
        if p <= m {
            self.upd(2 * v, l, m, p, x);
        } else {
            self.upd(2 * v + 1, m + 1, r, p, x);
        }
        self.t[v] = self.t[2 * v] + self.t[2 * v + 1];
    }
}
