use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::huffman::tree::TreeNode;

pub fn build_tree(data: &Vec<usize>, show_info: bool) -> TreeNode {
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

    if show_info {
        let mut avg = 0.;

        let codes = trees[0].0.codes();

        for key in alphabet.keys() {
            avg += *alphabet.get(key).unwrap() as f64 * codes.get(key).unwrap().len() as f64
                / data.len() as f64;
        }

        println!("Average bits for symbol: {:.2}", avg);
    }

    trees[0].0.clone()
}
