use std::cmp::max;

fn get(v: usize, p: &mut Vec<usize>) -> usize {
    if p[v] == v {
        return v;
    }
    p[v] = get(p[v], p);
    p[v]
}

fn merge(a: usize, b: usize, p: &mut Vec<usize>, s: &mut Vec<usize>) {
    let a = get(a, p);
    let b = get(b, p);
    if a == b {
        return;
    }
    if s[a] > s[b] {
        std::mem::swap(&mut a, &mut b);
    }
    p[a] = b;
    s[b] += s[a];
}

fn op1(x: usize, y: usize, p1: &mut Vec<usize>, s1: &mut Vec<usize>) {
    merge(x, y, p1, s1);
}

fn push(v: usize, p2: &mut Vec<usize>, r: &mut Vec<usize>) {
    if p2[v] == v {
        return;
    }
    r[p2[v]] = max(r[p2[v]], r[v]);
    push(p2[v], p2, r);
}

fn op2(x: usize, y: usize, p1: &mut Vec<usize>, s1: &mut Vec<usize>, p2: &mut Vec<usize>, s2: &mut Vec<usize>, r: &mut Vec<usize>) {
    let mut x = x;
    while x < y {
        merge(x, x + 1, p1, s1);
        merge(x, x + 1, p2, s2);
        push(x + 1, p2, r);
        x = r[get(x + 1, p2)];
    }
}

fn main() {
    let n = 0;
    let mut p1 = vec![1; n];
    let mut p2 = vec![1; n];
    let mut s1 = vec![1; n];
    let mut s2 = vec![1; n];
    let mut r = vec![1; n];

    for i in 0..n {
        p1[i] = i;
        p2[i] = i;
        r[i] = i;
    }
}
