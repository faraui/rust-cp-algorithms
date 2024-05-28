pub fn factmod(mut u: usize, v: usize) -> usize {
    let mut r = 1;
    while u > 1 {
        r *= ((u / v) % 2 == 1).then(|| v - 1).unwrap_or(1);
        r %= v;
        for i in 2 ..= u % v {
            r *= i;
            r %= v;
        }
        u /= v;
    }
    r % v
}
