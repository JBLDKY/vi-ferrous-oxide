#![allow(dead_code)]

use env_logger::Logger;
use log::debug;

const MAX_LEAF_SIZE: usize = 2;

#[derive(Debug, Clone)]
enum RopeNode {
    Leaf(String),
    // Left, Right, Length of the sum of left subnode strings.
    Branch(Box<RopeNode>, Box<RopeNode>, usize),
}

#[derive(Debug, Clone, Default)]
struct Rope {
    root: Option<RopeNode>,
}

impl Rope {
    // pub fn delete(&mut self, start_idx: usize, mut end_idx: usize) {
    //     if end_idx <= start_idx {
    //         return;
    //     }
    //
    //     if end_idx > self.len() {
    //         end_idx = self.len();
    //     }
    //
    //     if let Some(node) = self.root.take() {
    //         self.root = Some(Rope::delete_at_node(node, start_idx, end_idx));
    //     }
    // }

    // fn delete_at_node(node: RopeNode, start_idx: usize, end_idx: usize) -> RopeNode {
    //     match node {
    //         RopeNode::Leaf(s) => {
    //             if start_idx >= s.len() {
    //                 RopeNode::Leaf(s)
    //             } else {
    //                 let new_string = [&s[..start_idx], &s[end_idx..]].concat();
    //                 RopeNode::Leaf(new_string)
    //             }
    //         }
    //
    //         RopeNode::Branch(left, right, left_length) => {
    //             if end_idx <= left_length {
    //                 // Deletion starts and finishes on the left node
    //                 let new_left = Box::new(Rope::delete_at_node(*left, start_idx, end_idx));
    //                 RopeNode::Branch(new_left, right, left_length - (end_idx - start_idx))
    //             } else if start_idx >= left_length {
    //                 // Deletion doesn't start on the left node
    //                 let new_right = Box::new(Rope::delete_at_node(
    //                     *right,
    //                     start_idx - left_length,
    //                     end_idx - left_length,
    //                 ));
    //                 RopeNode::Branch(left, new_right, left_length)
    //             } else {
    //                 // Deletion range spans across left and right children
    //                 let new_left = Box::new(Rope::delete_at_node(*left, start_idx, left_length));
    //                 let new_right =
    //                     Box::new(Rope::delete_at_node(*right, 0, end_idx - left_length));
    //                 RopeNode::Branch(new_left, new_right, left_length - (end_idx - start_idx))
    //             }
    //         }
    //     }
    // }

    /// Concatenate two ropes
    ///
    /// Probably one of the few functions that do not require recursion
    pub fn concat(rope1: Rope, rope2: Rope) -> Rope {
        match (rope1.root, rope2.root) {
            (None, None) => Rope::new(),
            (Some(node), None) | (None, Some(node)) => Rope { root: Some(node) },
            (Some(left), Some(right)) => {
                let left_length = Self::traverse_and_get_len(&left);
                Rope {
                    root: Some(RopeNode::Branch(
                        Box::new(left),
                        Box::new(right),
                        left_length,
                    )),
                }
            }
        }
    }

    /// Split a rope into two ropes at the given index.
    pub fn split(self, mut idx: usize) -> (Rope, Rope) {
        if self.len() < idx {
            idx = self.len()
        }

        match self.root {
            None => (Rope::new(), Rope::new()),
            Some(node) => {
                let (left_node, right_node) = Rope::split_at_node(node, idx);
                (
                    Rope {
                        root: Some(left_node),
                    },
                    Rope {
                        root: Some(right_node),
                    },
                )
            }
        }
    }

    /// recursively find index and then split
    fn split_at_node(node: RopeNode, idx: usize) -> (RopeNode, RopeNode) {
        match node {
            RopeNode::Leaf(s) => {
                let (left, right) = s.split_at(idx);
                (
                    RopeNode::Leaf(left.to_string()),
                    RopeNode::Leaf(right.to_string()),
                )
            }

            RopeNode::Branch(left, right, left_length) => {
                if idx <= left_length {
                    let (new_left, split_off) = Rope::split_at_node(*left, idx);
                    (
                        new_left,
                        RopeNode::Branch(Box::new(split_off), right, left_length - idx),
                    )
                } else {
                    let (split_off, new_right) = Rope::split_at_node(*right, idx - left_length);
                    (
                        RopeNode::Branch(left, Box::new(split_off), left_length),
                        new_right,
                    )
                }
            }
        }
    }

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

    /// Recursively get the length of a node and its child nodes
    fn traverse_and_get_len(node: &RopeNode) -> usize {
        match node {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Branch(left, right, _) => {
                Self::traverse_and_get_len(left) + Self::traverse_and_get_len(right)
            }
        }
    }
    /// Get the string representation of the rope
    pub fn to_string(&self) -> Option<String> {
        // Recursively traverse the nodes and return the text of each node
        self.root.as_ref().map(Self::traverse_and_collect_text)
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

    pub fn index(&self, idx: usize) -> Option<char> {
        self.root
            .as_ref()
            .and_then(|node| Self::traverse_and_find_nth(node, idx))
    }

    // Recursively find the character at the given index
    fn traverse_and_find_nth(node: &RopeNode, idx: usize) -> Option<char> {
        match node {
            RopeNode::Leaf(s) => s.chars().nth(idx),
            RopeNode::Branch(left, right, left_length) => {
                if idx < *left_length {
                    Self::traverse_and_find_nth(left, idx)
                } else {
                    Self::traverse_and_find_nth(right, idx - *left_length)
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
fn test_small() {
    let s = "abc";
    let rope = Rope::from_str(s);

    let res = rope.index(0);
    assert_eq!('a', res.unwrap());

    let res = rope.index(1);
    assert_eq!('b', res.unwrap());

    let res = rope.index(2);
    assert_eq!('c', res.unwrap());
}

#[test]
fn test_index() {
    let s = "123456789";
    let rope = Rope::from_str(s);
    assert_eq!('4', rope.index(3).unwrap());
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

#[test]
fn test_split() {
    let s = "hello world";
    let rope = Rope::from_str(s);
    let (hello, rest) = rope.split(5);
    let hello = hello.to_string().unwrap();
    let world = rest.split(1).1.to_string().unwrap();

    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
}

#[test]
fn test_split_big() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    let (left, right) = rope.split(74);
    assert_eq!(
        left.to_string().unwrap(),
        "hi there hello world there once was a kitten that had a chocolate ice crea",
    );
    assert_eq!(right.to_string().unwrap(), "m");
}

#[test]
fn test_split_oob() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    let (left, right) = rope.split(80);

    assert_eq!(
        left.to_string().unwrap(),
        "hi there hello world there once was a kitten that had a chocolate ice cream",
    );

    assert_eq!(right.to_string().unwrap(), "",);
}

#[test]
fn test_at_zero() {
    let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
    let rope = Rope::from_str(s);
    let (left, right) = rope.split(0);

    assert_eq!(
        right.to_string().unwrap(),
        "hi there hello world there once was a kitten that had a chocolate ice cream",
    );
    assert_eq!(left.to_string().unwrap(), "",);
}

// #[test]
// fn test_delete() {
//     let s = "hello world";
//     let mut rope = Rope::from_str(s);
//     rope.delete(5, 100);
//     assert_eq!("hello", rope.to_string().unwrap());
//
//     let s = "hello world";
//     let mut rope = Rope::from_str(s);
//     rope.delete(5, 7);
//     assert_eq!("helloorld", rope.to_string().unwrap());
//
//     let s = "hello world";
//     let mut rope = Rope::from_str(s);
//     rope.delete(0, 5);
//     dbg!(&rope);
//     rope.delete(1, 5);
//     assert_eq!(" ", rope.to_string().unwrap());
// }
