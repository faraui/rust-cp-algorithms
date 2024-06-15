use std::cmp::max;

fn main() {
    let n: usize = 10;
    let m: usize = 10;
    let mut a: Vec<Vec<usize>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            a[i][j] = i * 10 + j;
        }
    }

    let log2_max = (max(n, m) as f64).log2() as usize + 1;
    let mut st: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; m]; n]; log2_max];

    for i in 0..n {
        for j in 0..m {
            st[0][i][j] = a[i][j];
        }
    }

    for i in 0..n {
        for k in 1..log2_max + 1 {
            for j in 0..m {
                if i + (1 << k) > n || j + (1 << k) > m {
                    break;
                }
                st[k][i][j] = max(
                    max(st[k - 1][i][j], st[k - 1][i][j + (1 << (k - 1))]),
                    max(
                        st[k - 1][i + (1 << (k - 1))][j],
                        st[k - 1][i + (1 << (k - 1))][j + (1 << (k - 1))],
                    ),
                );
            }
        }
    }
}
