use std::vec::Vec;

struct FenwickTree2D {
    t: Vec<Vec<usize>>,
}

impl FenwickTree2D {
    fn new(n: usize, m: usize) -> Self {
        FenwickTree2D {
            t: vec![vec![0; m + 1]; n + 1],
        }
    }

    fn upd(&mut self, p1: usize, p2: usize, x: usize) {
        let mut p1 = p1;
        let mut p2 = p2;
        while p1 < self.t.len() {
            let mut p2 = p2;
            while p2 < self.t[0].len() {
                self.t[p1][p2] += x;
                p2 += p2 & (!p2 + 1);
            }
            p1 += p1 & (!p1 + 1);
        }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        let mut x = x;
        let mut y = y;
        let mut res = 0;
        while x > 0 {
            let mut y = y;
            while y > 0 {
                res += self.t[x][y];
                y -= y & (!y + 1);
            }
            x -= x & (!x + 1);
        }
        res
    }

    fn get_rect(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
        self.get(x2, y2)
            - self.get(x1 - 1, y2)
            - self.get(x2, y1 - 1)
            + self.get(x1 - 1, y1 - 1)
    }
}
