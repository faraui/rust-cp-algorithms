pub fn f<const LEN: usize = 1>(n: usize) {
    if n >= LEN {
        f::<usize::min(1_000_000, LEN * 2)>(n);
    }
    let bs: [bool; LEN] = [false; LEN];
}
