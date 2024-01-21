#![allow(dead_code)]

const MAX_LEAF_SIZE: usize = 8;

#[derive(Debug, Clone)]
enum RopeNode {
    Leaf(String),
    Branch(Box<RopeNode>, Box<RopeNode>, usize),
}

#[derive(Debug, Clone, Default)]
struct Rope {
    root: Option<RopeNode>,
}

impl Rope {
    /// Returns an empty rope
    pub fn new() -> Self {
        Rope { root: None }
    }

    /// Creater a rope from a string slice
    pub fn from_str(s: &str) -> Self {
        let nodes = split_string_to_nodes(s);
        let root = build_tree_from_nodes(nodes);
        Rope { root: Some(root) }
    }

    /// Returns the length of the rope
    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, length_of_node)
    }

    pub fn index(&self, idx: usize) -> Option<char> {
        self.root.as_ref().and_then(|node| char_at(node, idx))
    }

    pub fn insert(&mut self, idx: usize, text: &str) {
        if let Some(node) = self.root.take() {
            self.root = Some(Rope::insert_at_node(node, idx, text));
        } else {
            self.root = Some(RopeNode::Leaf(text.to_string()));
        }
    }

    fn insert_at_node(node: RopeNode, idx: usize, text: &str) -> RopeNode {
        match node {
            RopeNode::Leaf(s) => {
                if idx >= s.len() {
                    let new_text = s + text;
                    RopeNode::Leaf(new_text)
                } else {
                    let (start, end) = s.split_at(idx);
                    let new_text = start.to_string() + text + end;
                    RopeNode::Leaf(new_text)
                }
            }
            RopeNode::Branch(left, right, left_length) => {
                if idx < left_length {
                    let new_left = Box::new(Rope::insert_at_node(*left, idx, text));
                    RopeNode::Branch(new_left, right, left_length + text.len())
                } else {
                    let new_right = Box::new(Rope::insert_at_node(*right, idx - left_length, text));
                    RopeNode::Branch(left, new_right, left_length)
                }
            }
        }
    }
}

/// Create leaves from a stringslice
fn split_string_to_nodes(s: &str) -> Vec<RopeNode> {
    s.as_bytes()
        .chunks(MAX_LEAF_SIZE)
        .map(|chunk| RopeNode::Leaf(String::from_utf8_lossy(chunk).to_string()))
        .collect()
}

fn char_at(node: &RopeNode, idx: usize) -> Option<char> {
    match node {
        RopeNode::Leaf(s) => s.chars().nth(idx),
        RopeNode::Branch(left, right, left_length) => {
            if idx < *left_length {
                char_at(left, idx)
            } else {
                char_at(right, idx - left_length)
            }
        }
    }
}

/// Recursively get the length of a node and its child nodes
fn length_of_node(node: &RopeNode) -> usize {
    match node {
        RopeNode::Leaf(s) => s.len(),
        RopeNode::Branch(left, right, _) => length_of_node(left) + length_of_node(right),
    }
}

/// Initialize a balanced tree
#[allow(dead_code)]
fn build_tree_from_nodes(nodes: Vec<RopeNode>) -> RopeNode {
    match nodes.len() {
        0 => RopeNode::Leaf(String::new()),
        1 => nodes.into_iter().next().unwrap(),
        _ => {
            let mid = nodes.len() / 2;
            let (left, right) = nodes.split_at(mid);
            let left_node = build_tree_from_nodes(left.to_vec());
            let right_node = build_tree_from_nodes(right.to_vec());

            let left_length = length_of_node(&left_node);
            RopeNode::Branch(Box::new(left_node), Box::new(right_node), left_length)
        }
    }
}

// Further functions and implementations
#[cfg(test)]
#[test]
fn test_rope_length() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    assert_eq!(rope.len(), rope.len());
}

#[test]
fn test_rope_init() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    assert_eq!(rope.len(), rope.len());
}

#[test]
fn test_index() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    let res = rope.index(21);
    assert_eq!('t', res.unwrap());
}

#[test]
fn test_index_none() {
    let s = "";
    let rope = Rope::from_str(s);
    let res = rope.index(1);
    assert_eq!(None, res);
}
