use std::vec;

use super::{build_tree::build_tree, tree::TreeNode};

pub fn encode(data: &Vec<usize>, show_info: bool) -> (TreeNode, Vec<bool>) {
    let tree = build_tree(data, show_info);
    let codes = tree.codes();

    let mut answer: Vec<bool> = vec![];

    for item in data {
        let mut code = codes.get(item).unwrap().clone();

        answer.append(&mut code);
    }

    (tree, answer)
}
