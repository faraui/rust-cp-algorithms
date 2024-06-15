use std::io::{stdin, stdout, Write};

type d1 = Vec<usize>;
type d2 = Vec<d1>;

let mut p: d2;

fn get(v: usize, d: usize) -> usize {
    for i in (0..p[0].len()).rev() {
        if d & 1 << i != 0 {
            v = p[v][i];
        }
    }
    v
}

fn main() {
    let mut n: usize;
    stdin().read_line(&mut String::new()).unwrap();
    n = stdin().read_line(&mut String::new()).unwrap();
    n = n.trim().parse().unwrap();
    let k = (n as f64).log2() as usize + 1;
    p = vec![vec![0; k]; n];
    p[0][0] = 0;
    for i in 1..n {
        let mut tmp = String::new();
        stdin().read_line(&mut tmp).unwrap();
        p[i][0] = tmp.trim().parse().unwrap();
    }
    for i in 1..k {
        for j in 0..n {
            p[j][i] = p[p[j][i - 1]][i - 1];
        }
    }
    let mut v: usize;
    let mut d: usize;
    stdin().read_line(&mut String::new()).unwrap();
    v = stdin().read_line(&mut String::new()).unwrap();
    v = v.trim().parse().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
    d = stdin().read_line(&mut String::new()).unwrap();
    d = d.trim().parse().unwrap();
    print!("{}", get(v, d));
    stdout().flush().unwrap();
}
