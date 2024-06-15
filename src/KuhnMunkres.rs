use std::io::{stdin, Read};

fn dfs(
    v: usize,
    g: &mut Vec<Vec<usize>>,
    mt: &mut Vec<i32>,
    seen: &mut Vec<usize>,
    step: &mut usize,
    cnt: &mut usize,
) -> bool {
    if seen[v] == *step {
        return false;
    }
    seen[v] = *step;
    for i in &g[v] {
        if mt[*i] == -1 {
            mt[*i] = v as i32;
            return true;
        }
    }
    for i in &g[v] {
        if dfs(mt[*i] as usize, g, mt, seen, step, cnt) {
            mt[*i] = v as i32;
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let (n, m) = {
        let mut nums = lines.next().unwrap().split_whitespace();
        (nums.next().unwrap().parse::<usize>().unwrap(), nums.next().unwrap().parse::<usize>().unwrap())
    };
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut mt: Vec<i32> = vec![-1; m];
    let mut seen: Vec<usize> = vec![0; n];
    let mut cnt = 0;
    let mut step = 0;
    for _ in 0..m {
        let mut nums = lines.next().unwrap().split_whitespace();
        let a = nums.next().unwrap().parse::<usize>().unwrap() - 1;
        let b = nums.next().unwrap().parse::<usize>().unwrap() - 1;
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        step += 1;
        if dfs(i, &mut g, &mut mt, &mut seen, &mut step, &mut cnt) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
    for i in 0..m {
        if mt[i] != -1 {
            println!("{} {}", mt[i] + 1, i + 1);
        }
    }
}
