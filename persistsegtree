use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Node {
    l: Option<Rc<RefCell<Node>>>,
    r: Option<Rc<RefCell<Node>>>,
    val: usize,
}

struct PersistentSegmentTree {
    root: Option<Rc<RefCell<Node>>>,
    a: Vec<usize>,
}

impl PersistentSegmentTree {
    fn new(a: Vec<usize>) -> Self {
        let root = Self::build(None, 0, a.len() - 1, &a);
        PersistentSegmentTree { root, a }
    }

    fn build(
        parent: Option<Rc<RefCell<Node>>>,
        l: usize,
        r: usize,
        a: &Vec<usize>,
    ) -> Option<Rc<RefCell<Node>>> {
        if l == r {
            Some(Rc::new(RefCell::new(Node {
                l: None,
                r: None,
                val: a[l],
            })))
        } else {
            let m = (l + r) / 2;
            let left = Self::build(parent.clone(), l, m, a);
            let right = Self::build(parent.clone(), m + 1, r, a);

            let mut node = Node {
                l: left,
                r: right,
                val: 0,
            };

            if let (Some(left), Some(right)) = (&node.l, &node.r) {
                node.val = left.borrow().val + right.borrow().val;
            }

            Some(Rc::new(RefCell::new(node)))
        }
    }

    fn get(
        &self,
        v: &Option<Rc<RefCell<Node>>>,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
    ) -> usize {
        if let Some(v) = v {
            let v = v.borrow();
            if r < ql || qr < l {
                0
            } else if ql <= l && r <= qr {
                v.val
            } else {
                let m = (l + r) / 2;
                Self::get(&v.l, l, m, ql, qr) + Self::get(&v.r, m + 1, r, ql, qr)
            }
        } else {
            0
        }
    }

    fn upd(
        &self,
        v: &Option<Rc<RefCell<Node>>>,
        l: usize,
        r: usize,
        p: usize,
        x: usize,
    ) -> Option<Rc<RefCell<Node>>> {
        if let Some(v) = v {
            let v = v.borrow();
            if l == r {
                Some(Rc::new(RefCell::new(Node {
                    l: None,
                    r: None,
                    val: x,
                })))
            } else {
                let m = (l + r) / 2;
                if p <= m {
                    let left = Self::upd(&v.l, l, m, p, x);
                    let right = v.r.clone();
                    let mut node = Node {
                        l: left,
                        r: right,
                        val: 0,
                    };
                    if let (Some(left), Some(right)) = (&node.l, &node.r) {
                        node.val = left.borrow().val + right.borrow().val;
                    }
                    Some(Rc::new(RefCell::new(node)))
                } else {
                    let left = v.l.clone();
                    let right = Self::upd(&v.r, m + 1, r, p, x);
                    let mut node = Node {
                        l: left,
                        r: right,
                        val: 0,
                    };
                    if let (Some(left), Some(right)) = (&node.l, &node.r) {
                        node.val = left.borrow().val + right.borrow().val;
                    }
                    Some(Rc::new(RefCell::new(node)))
                }
            }
        } else {
            None
        }
    }
}
