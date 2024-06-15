// Exact value
pub fn bc(n: usize, k: usize) -> usize {
    let k = if k > n - k { n - k } else { k };
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}

// Modulo M
pub fn bc(n: usize, k: usize, m: usize) -> usize {
    let fact_n = fact[n] % m;
    let fact_k = fact[k] % m;
    let fact_n_k = fact[n - k] % m;
    fact_n * binpow(fact_k, m - 2, m) % m * binpow(fact_n_k, m - 2, m) % m
}
