use std::collections::HashMap;

fn sufarr(s: &str) {
    s.push(0);
    let n = s.len();
    let mut c = vec![0; n];
    let mut p = vec![0; n];
    let mut cnt = 0;
    let mut cls = 0;
    let mut mp = HashMap::new();
    for i in 0..n {
        let ch = s.chars().nth(i).unwrap() as u8;
        mp.entry(ch).or_insert_with(Vec::new).push(i);
    }
    for (_, v) in mp.iter() {
        for &j in v {
            c[j] = cls;
            p[cnt] = j;
            cnt += 1;
        }
        cls += 1;
    }
    for l in 1.. {
        let d = 1 << (l / 2);
        let mut a = vec![Vec::new(); cls + 1];
        let mut c1 = vec![0; n];
        cls = -1;
        cnt = 0;
        for i in 0..n {
            let x = (p[i] - d + n) % n;
            a[c[x]].push(x);
        }
        for i in a {
            for (j, &val) in i.iter().enumerate() {
                if j == 0 || c[(val + d) % n] != c[(i[j - 1] + d) % n] {
                    cls += 1;
                }
                c1[val] = cls;
                p[cnt] = val;
                cnt += 1;
            }
        }
        c = c1;
        if cls + 1 >= n {
            break;
        }
    }
}
