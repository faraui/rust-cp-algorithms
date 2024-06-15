use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, Clone, Copy)]
struct R {
    x: usize,
    y: usize,
}

impl R {
    fn new(x: usize, y: usize) -> Self {
        R { x, y }
    }
}

fn len(a: R) -> usize {
    a.x * a.x + a.y * a.y
}

impl std::ops::Add for R {
    type Output = R;

    fn add(self, rhs: R) -> Self::Output {
        R::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for R {
    type Output = R;

    fn sub(self, rhs: R) -> Self::Output {
        R::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn operator_xor(a: R, b: R) -> usize {
    a.x * b.y - b.x * a.y
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut words = buffer.split_whitespace();
    let _ = words.next();
    let n: usize = words.next().unwrap().parse().unwrap();
    let mut a: Vec<R> = vec![R::new(0, 0); n];
    for i in 0..n {
        a[i].x = words.next().unwrap().parse().unwrap();
        a[i].y = words.next().unwrap().parse().unwrap();
    }
    let mut seen = vec![false; n];
    let mut cur_p = usize::MAX;
    for i in 0..n {
        if cur_p == usize::MAX || a[i].y < a[cur_p].y {
            cur_p = i;
        } else if a[i].y == a[cur_p].y && a[i].x < a[cur_p].x {
            cur_p = i;
        }
    }
    seen[cur_p] = true;
    let mut st = cur_p;
    loop {
        let mut new_p = usize::MAX;
        for i in 0..n {
            if !seen[i] {
                new_p = i;
                break;
            }
        }
        for i in 0..n {
            let aa = a[new_p] - a[cur_p];
            let b = a[i] - a[cur_p];
            let x = operator_xor(aa, b);
            if (!seen[i] || (i == st && st != cur_p)) && (x < 0 || (x == 0 && len(b) > len(aa))) {
                new_p = i;
            }
        }
        cur_p = new_p;
        if cur_p == st || cur_p == usize::MAX {
            break;
        }
        seen[cur_p] = true;
    }
}
