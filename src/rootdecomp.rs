use std::vec::Vec;

const C: usize = 350;

struct RootDecomposition {
    a: Vec<usize>,
    b: Vec<usize>,
    add: Vec<usize>,
    n: usize,
}

impl RootDecomposition {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        let mut b = vec![0; (n + C - 1) / C];
        let mut add = vec![0; (n + C - 1) / C];

        for i in 0..n {
            b[i / C] = b[i / C].max(a[i]);
        }

        RootDecomposition { a, b, add, n }
    }

    fn get(&self, l: usize, r: usize) -> usize {
        let mut res = 0;
        let mut l = l;
        let mut r = r;

        while l <= r {
            if l % C == 0 && l + C - 1 <= r {
                res = res.max(self.b[l / C]);
                l += C;
            } else {
                res = res.max(self.a[l] + self.add[l / C]);
                l += 1;
            }
        }

        res
    }

    fn upd(&mut self, l: usize, r: usize, x: usize) {
        let mut l = l;
        let mut r = r;

        while l <= r {
            if l % C == 0 && l + C - 1 <= r {
                self.b[l / C] += x;
                self.add[l / C] += x;
                l += C;
            } else {
                self.a[l] += x;
                self.b[l / C] = self.b[l / C].max(self.a[l]);
                l += 1;
            }
        }
    }
}
