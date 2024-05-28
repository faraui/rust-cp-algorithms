pub fn powmod(mut u: usize, mut v: usize, w: usize) -> usize {
    let mut r = 1;
    while v != 0 {
        if v & 1 != 0 {
            r *= u;
            r %= w;
        }
        u *= u;
        u %= w;
        v >>= 1;
    }
    r
}
