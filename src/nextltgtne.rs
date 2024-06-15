use std::collections::VecDeque;

fn next_lt(a: &Vec<usize>) -> Vec<usize> {
  let mut b = vec![0; a.len()];
  let mut s = VecDeque::new();
  for i in (0..a.len()).rev() {
    while !s.is_empty() && a[i] <= a[s.back().unwrap()] {
      s.pop_back();
    }
    if s.is_empty() {
      b[i] = usize::MAX;
    } else {
      b[i] = *s.back().unwrap();
    }
    s.push_back(i);
  }
  b
}

fn next_gt(a: &Vec<usize>) -> Vec<usize> {
  let mut b = vec![0; a.len()];
  let mut s = VecDeque::new();
  for i in (0..a.len()).rev() {
    while !s.is_empty() && a[i] >= a[s.back().unwrap()] {
      s.pop_back();
    }
    if s.is_empty() {
      b[i] = usize::MAX;
    } else {
      b[i] = *s.back().unwrap();
    }
    s.push_back(i);
  }
  b
}

fn next_ne(a: &Vec<usize>) -> Vec<usize> {
  let mut b = vec![0; a.len()];
  let mut s = VecDeque::new();
  for i in (0..a.len()).rev() {
    while !s.is_empty() && a[i] == a[s.back().unwrap()] {
      s.pop_back();
    }
    if s.is_empty() {
      b[i] = usize::MAX;
    } else {
      b[i] = *s.back().unwrap();
    }
    s.push_back(i);
  }
  b
}
