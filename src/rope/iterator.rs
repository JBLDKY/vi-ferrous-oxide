use crate::rope::base::RopeNode;
use crate::Rope;

pub struct RopeIterator<'a> {
    pub nodes: Vec<&'a RopeNode>,
    pub leaf_pos: usize,
    pub current_leaf: Option<&'a str>,
}


impl<'a> Iterator for RopeIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(leaf_str) = self.current_leaf {
                if self.leaf_pos < leaf_str.len() {
                    let char = leaf_str[self.leaf_pos..].chars().next().unwrap();
                    self.leaf_pos += char.len_utf8();
                    return Some(char);
                }
            }

            self.current_leaf = None;
            let next_node = self.nodes.pop()?;

            match next_node {
                RopeNode::Leaf(text) => {
                    self.current_leaf = Some(text);
                    self.leaf_pos = 0;
                }
                RopeNode::Branch(left, right, _) => {
                    self.nodes.push(right);
                    self.nodes.push(left);
                }
            }
        }
    }
}

