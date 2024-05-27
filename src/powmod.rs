pub fn powmod(mut u: u64, mut v: u64, w: u64) -> u64 {
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