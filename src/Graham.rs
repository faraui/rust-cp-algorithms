use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone, Copy)]
struct Pt {
    x: usize,
    y: usize,
}

fn cmp(a: Pt, b: Pt) -> Ordering {
    if a.x < b.x {
        Ordering::Less
    } else if a.x == b.x {
        if a.y < b.y {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}

fn cw(a: Pt, b: Pt, c: Pt) -> bool {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) < 0
}

fn ccw(a: Pt, b: Pt, c: Pt) -> bool {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) > 0
}

fn convex_hull(a: &mut Vec<Pt>) {
    if a.len() == 1 {
        return;
    }
    a.sort_by(cmp);
    let p1 = a[0];
    let p2 = a[a.len() - 1];
    let mut up = vec![p1];
    let mut down = vec![p1];
    for i in 1..a.len() {
        if i == a.len() - 1 || cw(p1, a[i], p2) {
            while up.len() >= 2 && !cw(up[up.len() - 2], up[up.len() - 1], a[i]) {
                up.pop();
            }
            up.push(a[i]);
        }
        if i == a.len() - 1 || ccw(p1, a[i], p2) {
            while down.len() >= 2 && !ccw(down[down.len() - 2], down[down.len() - 1], a[i]) {
                down.pop();
            }
            down.push(a[i]);
        }
    }
    a.clear();
    for i in 0..up.len() {
        a.push(up[i]);
    }
    for i in (1..down.len() - 1).rev() {
        a.push(down[i]);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut a = vec![Pt { x: 0, y: 0 }; n];
    for i in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        a[i].x = iter.next().unwrap().parse().unwrap();
        a[i].y = iter.next().unwrap().parse().unwrap();
    }
    convex_hull(&mut a);
    println!("{}", a.len());
    for i in a {
        println!("{} {}", i.x, i.y);
    }
    let mut res = 0;
    for i in 0..(a.len() - 1) {
        res += a[i].x * a[i + 1].y;
        res -= a[i + 1].x * a[i].y;
    }
    res += a[a.len() - 1].x * a[0].y;
    res -= a[0].x * a[a.len() - 1].y;
    print!("{}", res / 2);
    if res % 2 == 1 {
        print!(".5");
    }
    stdout().flush().unwrap();
}
