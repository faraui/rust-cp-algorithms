pub fn powmod(mut u: u128, mut v: u128, w: u128) -> u128 {
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
