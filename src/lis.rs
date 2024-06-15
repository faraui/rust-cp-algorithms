// Longest increasing subsequence

use std::io::{stdin, stdout, Write};
use std::cmp::{max, min};

fn main() {
    let mut n: usize = 0;
    stdin().read_line(&mut String::new()).unwrap();
    stdin().read_line(&mut format!("{}n", &mut n)).unwrap();
    n = n.trim().parse().unwrap();
    let mut a: Vec<i64> = vec![0; n];
    for i in 0..n {
        let mut s: String = String::new();
        stdin().read_line(&mut s).unwrap();
        a[i] = s.trim().parse().unwrap();
    }
    let mut dp: Vec<i64> = vec![1e18; n + 1];
    let mut pos: Vec<usize> = vec![0; n + 1];
    let mut prev: Vec<usize> = vec![0; n + 1];
    pos[0] = prev[0] = usize::MAX;
    dp[0] = -1e18;
    for i in 0..n {
        let j = dp.upper_bound(a[i]);
        if dp[j - 1] < a[i] && a[i] < dp[j] {
            dp[j] = a[i];
            pos[j] = i;
            prev[i] = pos[j - 1];
        }
    }
    let mut res: usize = 0;
    for i in (0..=n).rev() {
        if dp[i] != 1e18 {
            res = i;
            break;
        }
    }
    let mut p = pos[res];
    let mut ans: Vec<i64> = Vec::new();
    while p != usize::MAX {
        ans.push(a[p]);
        p = prev[p];
    }
    ans.reverse();
    println!("{}", res);
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    stdout().flush().unwrap();
}
