// https://codeforces.com/blog/entry/68953

const D: usize = 20;

let b: Vec<usize> = vec![0; D];

fn insert(mask: usize) {
  for i in 0..D {
    if !(mask & (1 << i)) {
      continue;
    }
    if b[i] == 0 {
      b[i] = mask;
      return;
    }
    mask ^= b[i];
  }
}

fn check(mask: usize) -> bool {
  for i in 0..D {
    if !(mask & (1 << i)) {
      continue;
    }
    if b[i] == 0 {
      return false;
    }
    mask ^= b[i];
  }
  true
}
