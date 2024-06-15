use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Query {
    l: usize,
    r: usize,
    id: usize,
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.l.cmp(&other.l))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        self.l.cmp(&other.l)
    }
}

const LEN: usize = 500;

fn main() {
    let mut n: usize;
    let mut m: usize;
    let mut a: Vec<usize>;
    let mut ans: Vec<usize>;
    let mut b: Vec<Vec<Query>>;
    let mut res: usize;

    scanf!(" %usize", &mut n);
    a = vec![0; n];
    b = vec![Vec::new(); LEN];
    for i in 0..n {
        scanf!(" %usize", &mut a[i]);
    }
    scanf!(" %usize", &mut m);
    ans = vec![0; m];
    for i in 0..m {
        let mut l: usize;
        let mut r: usize;
        scanf!(" %usize %usize", &mut l, &mut r);
        l -= 1;
        r -= 1;
        b[l / LEN].push(Query { l, r, id: i });
    }

    for i in 0..LEN {
        b[i].sort_by_key(|q| q.l);
    }

    res = 0;
    for i in 0..LEN {
        let mut l = i * LEN + 1;
        let mut r = i * LEN - 1;
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();

        for q in &b[i] {
            while r < q.r {
                r += 1;
                res += a[r];
                heap.push(r);
            }

            while r > q.r {
                res -= a[r];
                heap.pop();
                r -= 1;
            }

            while l < q.l {
                res -= a[l];
                heap.pop();
                l += 1;
            }

            while l > q.l {
                l -= 1;
                res += a[l];
                heap.push(l);
            }

            ans[q.id] = res;
        }
    }

    for i in ans {
        print!("{} ", i);
    }
    println!();
}

fn scanf<T: std::str::FromStr>(format: &str, value: &mut T) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    *value = input.trim().parse().unwrap();
}
