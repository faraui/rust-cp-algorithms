// https://neerc.ifmo.ru/wiki/index.php?title=%D0%91%D0%BE%D1%80

use std::collections::HashMap;

struct Node {
    to: HashMap<char, Box<Node>>,
    end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            to: HashMap::new(),
            end: false,
        }
    }
}

fn trie() {
    let mut root = Node::new();

    fn add(s: &str, p: &mut Node) {
        for c in s.chars() {
            if !p.to.contains_key(&c) {
                p.to.insert(c, Box::new(Node::new()));
            }
            p = p.to.get_mut(&c).unwrap();
        }
        p.end = true;
    }

    fn del(s: &str, p: &mut Node) {
        for c in s.chars() {
            if !p.to.contains_key(&c) {
                return;
            }
            p = p.to.get_mut(&c).unwrap();
        }
        p.end = false;
    }

    fn check(s: &str, p: &Node) -> bool {
        let mut p = p;
        for c in s.chars() {
            if !p.to.contains_key(&c) {
                return false;
            }
            p = p.to.get(&c).unwrap();
        }
        p.end
    }

    fn sort(p: &Node, s: &mut String) {
        if p.end {
            println!("{}", s);
        }
        for (c, node) in p.to.iter() {
            s.push(*c);
            sort(node, s);
            s.pop();
        }
    }
}
