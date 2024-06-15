use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use rand::prelude::*;

#[derive(Debug)]
struct Node {
    k: usize,
    p: usize,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl Node {
    fn new(k: usize) -> Self {
        let mut rng = rand::thread_rng();
        Node {
            k,
            p: rng.gen(),
            l: None,
            r: None,
        }
    }
}

fn split(t: Option<Box<Node>>, x: usize, l: &mut Option<Box<Node>>, r: &mut Option<Box<Node>>) {
    if let Some(mut t) = t {
        if t.k >= x {
            split(t.l.take(), x, l, &mut t.l);
            *r = Some(t);
        } else {
            split(t.r.take(), x, &mut t.r, r);
            *l = Some(t);
        }
    } else {
        *l = None;
        *r = None;
    }
}

fn merge(l: Option<Box<Node>>, r: Option<Box<Node>>, t: &mut Option<Box<Node>>) {
    if let Some(mut l) = l {
        if let Some(mut r) = r {
            if l.p > r.p {
                merge(l.r.take(), r, &mut l.r);
                *t = Some(l);
            } else {
                merge(l, r.l.take(), &mut r.l);
                *t = Some(r);
            }
        } else {
            *t = Some(l);
        }
    } else {
        *t = r;
    }
}

fn get_l(t: &Option<Box<Node>>) -> usize {
    if let Some(t) = t {
        if t.l.is_none() {
            t.k
        } else {
            get_l(&t.l)
        }
    } else {
        usize::MAX
    }
}

fn get_r(t: &Option<Box<Node>>) -> usize {
    if let Some(t) = t {
        if t.r.is_none() {
            t.k
        } else {
            get_r(&t.r)
        }
    } else {
        usize::MAX
    }
}

fn find(t: &Option<Box<Node>>, x: usize) -> bool {
    if let Some(t) = t {
        if t.k > x {
            find(&t.l, x)
        } else if t.k < x {
            find(&t.r, x)
        } else {
            true
        }
    } else {
        false
    }
}

let mut root: Option<Box<Node>> = None;

fn check(x: usize) -> bool {
    let mut l: Option<Box<Node>> = None;
    let mut r: Option<Box<Node>> = None;
    split(root.take(), x, &mut l, &mut r);
    let res = find(&r, x);
    merge(l, r, &mut root);
    res
}

fn read(p: &Option<Box<Node>>) {
    if let Some(p) = p {
        read(&p.l);
        print!("{} ", p.k);
        read(&p.r);
    }
}

fn add(x: usize) {
    let mut l: Option<Box<Node>> = None;
    let mut r: Option<Box<Node>> = None;
    split(root.take(), x, &mut l, &mut r);
    if !find(&r, x) {
        merge(l, Some(Box::new(Node::new(x))), &mut root);
        merge(root.take(), r, &mut root);
    } else {
        merge(l, r, &mut root);
    }
}

fn del(x: usize) {
    let mut l: Option<Box<Node>> = None;
    let mut r: Option<Box<Node>> = None;
    let mut m: Option<Box<Node>> = None;
    split(root.take(), x, &mut l, &mut r);
    split(r.take(), x + 1, &mut m, &mut r);
    merge(l, r, &mut root);
}

fn next(x: usize) -> usize {
    let mut l: Option<Box<Node>> = None;
    let mut r: Option<Box<Node>> = None;
    split(root.take(), x + 1, &mut l, &mut r);
    let y = get_l(&r);
    merge(l, r, &mut root);
    y
}

fn prev(x: usize) -> usize {
    let mut l: Option<Box<Node>> = None;
    let mut r: Option<Box<Node>> = None;
    split(root.take(), x, &mut l, &mut r);
    let y = get_r(&l);
    merge(l, r, &mut root);
    y
}

fn kth(p: &Option<Box<Node>>, x: usize) -> usize {
    if let Some(p) = p {
        let l = cnt(&p.l);
        if l == x {
            p.k
        } else if l > x {
            kth(&p.l, x)
        } else if l < x {
            kth(&p.r, x - l - 1)
        } else {
            usize::MAX
        }
    } else {
        usize::MAX
    }
}

fn cnt(p: &Option<Box<Node>>) -> usize {
    if let Some(p) = p {
        cnt(&p.l) + 1 + cnt(&p.r)
    } else {
        0
    }
}
