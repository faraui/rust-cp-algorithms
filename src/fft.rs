use std::f64::consts::PI;
use std::iter::repeat;
use std::ops::{AddAssign, MulAssign, SubAssign};

type Comp = complex::Complex<f64>;

fn fft(p: &mut Vec<Comp>, w: Comp, buff: &mut Vec<Vec<Vec<Comp>>>) {
    if p.len() == 1 {
        return;
    }
    let lg = (p.len() as f64).log2() as usize - 1;
    for i in 0..p.len() {
        if i & 1 == 1 {
            buff[lg][1][i / 2] = p[i];
        } else {
            buff[lg][0][i / 2] = p[i];
        }
    }
    fft(&mut buff[lg][0], w * w, buff);
    fft(&mut buff[lg][1], w * w, buff);
    let mut wk = Comp::new(1.0, 0.0);
    let k = p.len() / 2;
    for i in 0..p.len() {
        p[i] = buff[lg][0][i % k] + wk * buff[lg][1][i % k];
        wk *= w;
    }
}

fn get_w(n: usize) -> Comp {
    Comp::new(PI * 2.0 / n as f64).cos() + Comp::new(PI * 2.0 / n as f64).sin() * Comp::new(0.0, 1.0)
}

fn evalute(p: &Vec<usize>, buff: &mut Vec<Vec<Vec<Comp>>>) -> Vec<Comp> {
    let mut _p = p.iter().map(|&x| Comp::new(x as f64, 0.0)).collect::<Vec<Comp>>();
    fft(&_p, get_w(p.len()), buff);
    _p
}

fn interpolate(p: &Vec<Comp>, buff: &mut Vec<Vec<Vec<Comp>>>) -> Vec<usize> {
    fft(p, get_w(p.len()).conj(), buff);
    let mut _p = p.iter().map(|&x| (x.re / p.len() as f64).round() as usize).collect::<Vec<usize>>();
    _p
}

fn prepare(a: &mut Vec<usize>, b: &mut Vec<usize>) {
    let n = 1 << ((a.len() + b.len()) as f64).log2() as usize;
    a.resize_with(n, || 0);
    b.resize_with(n, || 0);
}

fn multiply(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut a = a;
    let mut b = b;
    a.reverse();
    b.reverse();
    prepare(&mut a, &mut b);
    let mut buff = repeat(vec![vec![Vec::new(); 2]; 2])
        .take((a.len() as f64).log2() as usize + 1)
        .collect::<Vec<Vec<Vec<Comp>>>>();
    for i in 0..buff.len() {
        buff[i][0].resize(1 << i, Comp::new(0.0, 0.0));
        buff[i][1].resize(1 << i, Comp::new(0.0, 0.0));
    }
    let _a = evalute(&a, &mut buff);
    let _b = evalute(&b, &mut buff);
    let mut _res = vec![Comp::new(0.0, 0.0); a.len()];
    for i in 0.._res.len() {
        _res[i] = _a[i] * _b[i];
    }
    let mut res = interpolate(&_res, &mut buff);
    for i in 0..res.len() - 1 {
        res[i + 1] += res[i] / 10;
        res[i] %= 10;
    }
    while res.iter().last().unwrap() >= &10 {
        let tmp = res.pop().unwrap();
        res.push(tmp % 10);
        res.push(tmp / 10);
    }
    while res.len() > 1 && *res.last().unwrap() == 0 {
        res.pop();
    }
    res.reverse();
    res
}

fn main() {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut _a = String::new();
    let mut _b = String::new();
    std::io::stdin().read_line(&_a).unwrap();
    std::io::stdin().read_line(&_b).unwrap();
    let mut f = 0;
    if _a.starts_with('-') {
        f += 1;
        _a.remove(0);
    }
    if _b.starts_with('-') {
        f += 1;
        _b.remove(0);
    }
    for i in _a.chars() {
        a.push(i.to_digit(10).unwrap() as usize);
    }
    for i in _b.chars() {
        b.push(i.to_digit(10).unwrap() as usize);
    }
    f %= 2;
    if a[0] == 0 || b[0] == 0 {
        f = 0;
    }
    let res = multiply(a, b);
    if f == 1 {
        print!("-");
    }
    for i in res {
        print!("{}", i);
    }
}
