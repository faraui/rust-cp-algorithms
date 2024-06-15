use std::vec::Vec;

struct SparseTable {
    st1: Vec<Vec<usize>>,
    st2: Vec<Vec<usize>>,
    k: usize,
    n: usize,
}

impl SparseTable {
    fn new() -> Self {
        SparseTable {
            st1: Vec::new(),
            st2: Vec::new(),
            k: 0,
            n: 0,
        }
    }

    fn from_vec(a: &Vec<usize>) -> Self {
        let n = a.len();
        let k = (n as f64).log2() as usize + 1;
        let mut st1 = vec![vec![0; n]; k];
        let mut st2 = vec![vec![0; n]; k];

        for i in 0..n {
            st1[0][i] = a[i];
            st2[0][i] = a[i];
        }

        for i in 1..k {
            for j in 0..(n - (1 << i) + 1) {
                st1[i][j] = st1[i - 1][j].min(st1[i - 1][j + (1 << (i - 1))]);
                st2[i][j] = st2[i - 1][j].max(st2[i - 1][j + (1 << (i - 1))]);
            }
        }

        SparseTable {
            st1,
            st2,
            k,
            n,
        }
    }

    fn get_min(&self, l: usize, r: usize) -> usize {
        let k = (r - l + 1 as usize).log2() as usize;
        self.st1[k][l].min(self.st1[k][r - (1 << k) + 1])
    }

    fn get_max(&self, l: usize, r: usize) -> usize {
        let k = (r - l + 1 as usize).log2() as usize;
        self.st2[k][l].max(self.st2[k][r - (1 << k) + 1])
    }
}
