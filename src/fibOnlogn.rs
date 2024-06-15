fn fib(n: usize, m: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut n = n - 1;
    let mut a = [[1, 1], [1, 0]];
    let mut res = [[1, 0], [0, 1]];
    let mut b = [[0, 0], [0, 0]];
    while n > 0 {
        if n & 1 == 1 {
            b[0][0] = (res[0][0] * a[0][0] + res[0][1] * a[1][0]) % m;
            b[0][1] = (res[0][0] * a[0][1] + res[0][1] * a[1][1]) % m;
            b[1][0] = (res[1][0] * a[0][0] + res[1][1] * a[1][0]) % m;
            b[1][1] = (res[1][0] * a[0][1] + res[1][1] * a[1][1]) % m;
            std::mem::swap(&mut res, &mut b);
        }
        b[0][0] = (a[0][0] * a[0][0] + a[0][1] * a[1][0]) % m;
        b[0][1] = (a[0][0] * a[0][1] + a[0][1] * a[1][1]) % m;
        b[1][0] = (a[1][0] * a[0][0] + a[1][1] * a[1][0]) % m;
        b[1][1] = (a[1][0] * a[0][1] + a[1][1] * a[1][1]) % m;
        std::mem::swap(&mut a, &mut b);
        n >>= 1;
    }
    res[0][0]
}
