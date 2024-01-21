#![allow(dead_code)]

const MAX_LEAF_SIZE: usize = 8;

#[derive(Debug, Clone)]
enum RopeNode {
    Leaf(String),
    // Left, Right, Length of left node
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
        let nodes = Self::split_string_to_nodes(s);
        let root = Self::build_tree_from_nodes(nodes);
        Rope { root: Some(root) }
    }

    /// Create leaves from a stringslice
    /// Used during initialization
    fn split_string_to_nodes(s: &str) -> Vec<RopeNode> {
        s.as_bytes()
            .chunks(MAX_LEAF_SIZE)
            .map(|chunk| RopeNode::Leaf(String::from_utf8_lossy(chunk).to_string()))
            .collect()
    }

    /// Initialize a balanced tree
    fn build_tree_from_nodes(nodes: Vec<RopeNode>) -> RopeNode {
        match nodes.len() {
            0 => RopeNode::Leaf(String::new()),
            1 => nodes.into_iter().next().unwrap(), // Return the singular node
            _ => {
                let mid = nodes.len() / 2;
                let (left, right) = nodes.split_at(mid);
                let left_node = Self::build_tree_from_nodes(left.to_vec());
                let right_node = Self::build_tree_from_nodes(right.to_vec());

                let left_length = Self::traverse_and_get_len(&left_node);
                RopeNode::Branch(Box::new(left_node), Box::new(right_node), left_length)
            }
        }
    }

    /// Get the string representation of the rope
    pub fn to_string(&self) -> Option<String> {
        // Recursively traverse the nodes and return the text of each node
        self.root
            .as_ref().map(Self::traverse_and_collect_text)
    }

    fn traverse_and_collect_text(node: &RopeNode) -> String {
        match node {
            // If we find a Leaf node, return the text
            RopeNode::Leaf(s) => s.to_string(),

            // If we find a branch, recurse
            RopeNode::Branch(left, right, _) => {
                format!(
                    "{}{}",
                    &Self::traverse_and_collect_text(left),
                    &Self::traverse_and_collect_text(right)
                )
            }
        }
    }

    /// Returns the length of the rope
    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, Self::traverse_and_get_len)
    }

    /// Recursively get the length of a node and its child nodes
    fn traverse_and_get_len(node: &RopeNode) -> usize {
        match node {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Branch(left, right, _) => {
                Self::traverse_and_get_len(left) + Self::traverse_and_get_len(right)
            }
        }
    }

    pub fn index(&self, idx: usize) -> Option<char> {
        self.root
            .as_ref()
            .and_then(|node| Self::traverse_and_find_nth(node, idx))
    }

    // Recursively find the character at the given index
    fn traverse_and_find_nth(node: &RopeNode, idx: usize) -> Option<char> {
        match node {
            // if we hit a Leaf, return the nth character
            RopeNode::Leaf(s) => s.chars().nth(idx),

            // if we hit a branch we might have to deduct from idx
            RopeNode::Branch(left, right, left_length) => {
                if idx < *left_length {
                    Self::traverse_and_find_nth(left, idx)
                } else {
                    Self::traverse_and_find_nth(right, idx - left_length)
                }
            }
        }
    }

    pub fn insert(&mut self, idx: usize, text: &str) {
        // Consume the root
        if let Some(node) = self.root.take() {
            // Recursively find index and insert
            self.root = Some(Rope::traverse_and_insert(node, idx, text));
        } else {
            // Insert when this is an empty string
            self.root = Some(RopeNode::Leaf(text.to_string()));
        }
    }

    fn traverse_and_insert(node: RopeNode, idx: usize, text: &str) -> RopeNode {
        match node {
            RopeNode::Leaf(s) => {
                if idx >= s.len() {
                    // Insert at end
                    let new_text = s + text;
                    RopeNode::Leaf(new_text)
                } else {
                    // Insert in the middle
                    let (start, end) = s.split_at(idx);
                    let new_text = start.to_string() + text + end;
                    RopeNode::Leaf(new_text)
                }
            }
            RopeNode::Branch(left, right, left_length) => {
                if idx < left_length {
                    // Must be the node where we find the correct index
                    let new_left = Box::new(Rope::traverse_and_insert(*left, idx, text));
                    RopeNode::Branch(new_left, right, left_length + text.len())
                } else {
                    // Not yet at the index
                    let new_right =
                        Box::new(Rope::traverse_and_insert(*right, idx - left_length, text));
                    RopeNode::Branch(left, new_right, left_length)
                }
            }
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

#[test]
fn test_insert_at() {
    let s = "hello world";
    let mut rope = Rope::from_str(s);
    rope.insert(5, " CTHULHU");
    assert_eq!(rope.to_string().unwrap(), "hello CTHULHU world")
}

#[test]
fn test_to_string() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    assert_eq!(s, rope.to_string().unwrap());
}
