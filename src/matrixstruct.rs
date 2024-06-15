use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::fmt::Display;

#[derive(Clone)]
struct Matrix {
    data: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}

impl Matrix {
    fn new(n: usize, m: usize) -> Self {
        Self {
            data: vec![vec![0; m]; n],
            n,
            m,
        }
    }

    fn from_vec(data: Vec<Vec<usize>>) -> Result<Self, &'static str> {
        let n = data.len();
        let m = data[0].len();
        if data.iter().all(|row| row.len() == m) {
            Ok(Self { data, n, m })
        } else {
            Err("Wrong matrix")
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<usize>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.n != rhs.n || self.m != rhs.m {
            panic!("Can't sum matrixes");
        }
        let mut res = Self::new(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] + rhs[i][j];
            }
        }
        res
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.n != rhs.n || self.m != rhs.m {
            panic!("Can't substract matrixes");
        }
        let mut res = Self::new(self.n, self.m);
        for i in 0..self.n {
            for j in 0..self.m {
                res[i][j] = self[i][j] - rhs[i][j];
            }
        }
        res
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.m != rhs.n {
            panic!("Can't multiply matrixes");
        }
        let mut res = Self::new(self.n, rhs.m);
        for i in 0..res.n {
            for j in 0..res.m {
                for k in 0..self.m {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{} ", self[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix {
    fn pow(&self, x: usize) -> Self {
        if self.n != self.m {
            panic!("Matrix is not square");
        }
        let mut a = self.clone();
        let mut res = Self::new(self.n, self.m);
        for i in 0..res.n {
            res[i][i] = 1;
        }
        if x == 0 {
            return res;
        }
        let mut x = x;
        while x > 0 {
            if x & 1 == 1 {
                res = res * a.clone();
            }
            a = a * a;
            x >>= 1;
        }
        res
    }
}
