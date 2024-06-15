const B: usize = 27;
const M: usize = 1000000007;

fn get(p: &Vec<usize>, h: &Vec<usize>, l: usize, r: usize) -> usize {
    (h[r + 1] - (h[l] * p[r - l + 1]) % M + M) % M
}

fn hash(s: &str) -> usize {
    let n = s.len();
    let mut p = vec![1; n + 1];
    let mut h = vec![0; n + 1];
    for i in 1..=n {
        p[i] = (p[i - 1] * B) % M;
        h[i] = (h[i - 1] * B + (s.as_bytes()[i - 1] - b'a' + 1) as usize) % M;
    }
    get(&p, &h, 0, 2)
}
