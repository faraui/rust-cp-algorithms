use std::collections::HashMap;

struct Node {
    to: HashMap<char, Box<Node>>,
    go: HashMap<char, Box<Node>>,
    p: Option<Box<Node>>,
    link: Option<Box<Node>>,
    up: Option<Box<Node>>,
    c: char,
    end: bool,
}

impl Node {
    fn new() -> Box<Node> {
        Box::new(Node {
            to: HashMap::new(),
            go: HashMap::new(),
            p: None,
            link: None,
            up: None,
            c: 0,
            end: false,
        })
    }

    fn new_with_c(c: char, p: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node {
            to: HashMap::new(),
            go: HashMap::new(),
            p,
            link: None,
            up: None,
            c,
            end: false,
        })
    }
}

fn go(p: &Box<Node>, c: char) -> &Box<Node> {
    if let Some(go) = p.go.get(&c) {
        return go;
    }
    if let Some(to) = p.to.get(&c) {
        p.go.insert(c, to.clone());
        return to;
    }
    if p.p.is_none() {
        p.go.insert(c, p.clone());
        return p;
    }
    let link = link(&p);
    let go = go(link, c);
    p.go.insert(c, go.clone());
    go
}

fn link(p: &Box<Node>) -> &Box<Node> {
    if let Some(link) = &p.link {
        return link;
    }
    if p.p.is_none() {
        p.link = Some(p.clone());
        return p;
    }
    let link = link(&p.p.as_ref().unwrap());
    let link = go(link, p.c);
    p.link = Some(link.clone());
    link
}

fn up(p: &Box<Node>) -> Option<&Box<Node>> {
    if let Some(up) = &p.up {
        return Some(up);
    }
    let link = link(p);
    if link.end {
        p.up = Some(link.clone());
        return Some(link);
    }
    if p.p.is_none() {
        p.up = None;
        return None;
    }
    let up = up(link);
    p.up = up.cloned();
    up
}

fn find(p: &Box<Node>) -> usize {
    let mut res = 0;
    let mut p = p;
    while let Some(p) = up(p) {
        if p.end {
            res += 1;
        }
    }
    res
}

fn add(s: &str, root: &Box<Node>) {
    let mut p = root;
    for c in s.chars() {
        if !p.to.contains_key(&c) {
            p.to.insert(c, Node::new_with_c(c, Some(p.clone())));
        }
        p = p.to.get(&c).unwrap();
    }
    p.end = true;
}

fn main(s: &str) {
    let root = Node::new();
    add("end", &root);
    add("one", &root);
    let mut p = &root;
    let mut res = 0;
    for c in s.chars() {
        p = go(&p, c);
        res += find(p);
    }
    println!("{}", res);
}
