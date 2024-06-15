use std::vec::Vec;

struct Seg {
    val: usize,
    push: usize,
}

struct SegmentTreeMassiveOperations {
    t: Vec<Seg>,
    a: Vec<usize>,
}

impl SegmentTree {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        let mut t = vec![Seg { val: 0, push: 0 }; 4 * n];
        let mut st = SegmentTree { t, a };
        st.build(1, 0, n - 1);
        st
    }

    fn build(&mut self, v: usize, l: usize, r: usize) {
        if l == r {
            self.t[v].val = self.a[l];
        } else {
            let m = (l + r) / 2;
            self.build(2 * v, l, m);
            self.build(2 * v + 1, m + 1, r);
            self.t[v].val = self.t[2 * v].val + self.t[2 * v + 1].val;
        }
    }

    fn push(&mut self, v: usize, l: usize, r: usize) {
        if self.t[v].push == 0 {
            return;
        }
        self.t[v].val += self.t[v].push * (r - l + 1);
        if l != r {
            self.t[2 * v].push += self.t[v].push;
            self.t[2 * v + 1].push += self.t[v].push;
        }
        self.t[v].push = 0;
    }

    fn upd(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize, x: usize) {
        self.push(v, l, r);
        if r < ql || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            self.t[v].push += x;
            self.push(v, l, r);
            return;
        }
        let m = (l + r) / 2;
        self.upd(2 * v, l, m, ql, qr, x);
        self.upd(2 * v + 1, m + 1, r, ql, qr, x);
        self.t[v].val = self.t[2 * v].val + self.t[2 * v + 1].val;
    }

    fn get(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> usize {
        self.push(v, l, r);
        if r < ql || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.t[v].val;
        }
        let m = (l + r) / 2;
        self.get(2 * v, l, m, ql, qr) + self.get(2 * v + 1, m + 1, r, ql, qr)
    }
}
