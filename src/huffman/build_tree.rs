use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::huffman::tree::TreeNode;

pub fn build_tree(data: &Vec<usize>) -> TreeNode {
    let mut alphabet: HashMap<usize, usize> = HashMap::new();

    for item in data {
        *alphabet.entry(*item).or_insert(0) += 1;
    }

    let mut trees: Vec<(TreeNode, usize)> = alphabet
        .iter()
        .map(|item| (TreeNode::new(Some(*item.0)), *item.1))
        .collect();

    while trees.len() > 1 {
        trees.sort_by(|a, b| b.1.cmp(&a.1));

        let first = trees.pop().unwrap();
        let second = trees.pop().unwrap();

        let mut tree = TreeNode::new(None);

        tree.push(Rc::new(RefCell::new(first.0)));
        tree.push(Rc::new(RefCell::new(second.0)));

        trees.push((tree, first.1 + second.1));
    }

    trees[0].0.clone()
}
