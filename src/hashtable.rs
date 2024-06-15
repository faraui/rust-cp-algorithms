use std::collections::VecDeque;

fn h1(x: usize) -> usize {
    x % m
}

fn h2(x: usize) -> usize {
    1 + x % (m - 2)
}

struct HashTable {
    val: VecDeque<usize>,
    t: VecDeque<usize>,
    tombstone: VecDeque<bool>,
    m: usize,
    is_multi: bool,
}

impl HashTable {
    fn new(m: usize, is_multi: bool) -> Self {
        Self {
            val: VecDeque::with_capacity(m),
            t: VecDeque::with_capacity(m),
            tombstone: VecDeque::with_capacity(m),
            m,
            is_multi,
        }
    }

    fn add(&mut self, p: usize) {
        let mut i = h1(p);
        let d = h2(p);
        while self.t[i] || self.tombstone[i] {
            if self.val[i] == p || self.tombstone[i] {
                self.val[i] = p;
                self.t[i] = if self.is_multi { self.t[i] + 1 } else { 1 };
                self.tombstone[i] = false;
                return;
            }
            i = (i + d) % self.m;
        }
        self.t[i] = 1;
        self.val[i] = p;
    }

    fn get(&self, p: usize) -> usize {
        let mut i = h1(p);
        let d = h2(p);
        while self.t[i] || self.tombstone[i] {
            if self.val[i] == p {
                return if self.is_multi { self.t[i] } else { if self.t[i] > 0 { 1 } else { 0 } };
            }
            i = (i + d) % self.m;
        }
        0
    }

    fn del(&mut self, p: usize) {
        let mut i = h1(p);
        let d = h2(p);
        while self.t[i] || self.tombstone[i] {
            if self.val[i] == p {
                if self.t[i] > 0 { self.t[i] -= 1; }
                if self.t[i] == 0 { self.tombstone[i] = true; }
                return;
            }
            i = (i + d) % self.m;
        }
    }
}
