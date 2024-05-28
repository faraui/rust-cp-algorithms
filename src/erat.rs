fn erat(u: usize) -> Vec<usize> {
    let mut p = vec![0; u + 1];
    let mut primes = Vec::new();
    for i in 2 ..= u {
        if p[i] == 0 {
            p[i] = i;
            primes.push(i);
        }
        for j in primes.iter() {
            if i * j > u || j > &p[i] {
                break;
            }
            p[i * j] = *j;
        }
    }
    primes
}
