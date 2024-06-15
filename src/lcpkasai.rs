fn kasai(s: &str, p: &Vec<usize>) -> Vec<usize> {
    let n = s.len();
    let mut lcp = vec![0; n];
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[p[i]] = i;
    }
    let mut k = 0;
    for i in 0..n {
        if k > 0 {
            k -= 1;
        }
        if pos[i] == n - 1 {
            k = 0;
            continue;
        }
        let j = p[pos[i] + 1];
        while i + k < n && j + k < n && s.chars().nth(i + k).unwrap() == s.chars().nth(j + k).unwrap() {
            k += 1;
        }
        lcp[pos[i]] = k;
    }
    lcp
}
