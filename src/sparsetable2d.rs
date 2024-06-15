use std::cmp::min;

fn main() {
    let n: usize = 10;
    let m: usize = 10;
    let mut a: Vec<Vec<usize>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            a[i][j] = i * 10 + j;
        }
    }

    let log2_n = (n as f64).log2() as usize + 1;
    let log2_m = (m as f64).log2() as usize + 1;
    let mut st: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; m]; n]; log2_m]; log2_n];

    for i in 0..n {
        for j in 0..m {
            st[0][0][i][j] = a[i][j];
        }
    }

    for _i in 0..log2_n {
        for _j in 0..log2_m {
            for i in 0..n {
                for j in 0..m {
                    st[_i + 1][_j + 1][i][j] = min(
                        min(st[_i][_j][i][j], st[_i][_j][i + (1 << _i)][j]),
                        min(st[_i][_j][i][j + (1 << _j)], st[_i][_j][i + (1 << _i)][j + (1 << _j)]),
                    );
                }
            }
        }
    }
}
