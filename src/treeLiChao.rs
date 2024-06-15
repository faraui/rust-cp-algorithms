struct Line {
    k: usize,
    b: usize,
}

impl Line {
    fn new() -> Self {
        Self { k: 0, b: 0 }
    }

    fn from(k: usize, b: usize) -> Self {
        Self { k, b }
    }

    fn get(&self, x: f64) -> usize {
        (self.k as f64 * x + self.b as f64) as usize
    }
}

#[derive(Debug, Clone)]
struct Node {
    line: Line,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(line: Line) -> Self {
        Self {
            line,
            left: None,
            right: None,
        }
    }
}

fn upd(node: &mut Node, l: usize, r: usize, line: Line) {
    let m = (l + r) / 2;
    let left = line.get(l as f64) > node.line.get(l as f64);
    let mid = line.get(m as f64) > node.line.get(m as f64);
    if mid {
        std::mem::swap(&mut node.line, &mut line);
    }
    if l == r {
        return;
    }
    if left != mid {
        if let Some(ref mut left) = node.left {
            upd(left, l, m, line);
        } else {
            node.left = Some(Box::new(Node::new(line)));
        }
    } else {
        if let Some(ref mut right) = node.right {
            upd(right, m + 1, r, line);
        } else {
            node.right = Some(Box::new(Node::new(line)));
        }
    }
}

fn get(node: &Node, l: usize, r: usize, x: usize) -> usize {
    if l == r {
        return node.line.get(x as f64);
    }
    let m = (l + r) / 2;
    if x < m {
        if let Some(ref left) = node.left {
            return std::cmp::max(node.line.get(x as f64), get(left, l, m, x));
        } else {
            return node.line.get(x as f64);
        }
    } else {
        if let Some(ref right) = node.right {
            return std::cmp::max(node.line.get(x as f64), get(right, m + 1, r, x));
        } else {
            return node.line.get(x as f64);
        }
    }
}
