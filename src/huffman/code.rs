use std::vec;

use super::{build_tree::build_tree, tree::TreeNode};

pub fn code(data: &Vec<usize>) -> (TreeNode, Vec<bool>) {
    let tree = build_tree(data);
    let codes = tree.codes();

    let mut answer: Vec<bool> = vec![];

    for item in data {
        let mut code = codes.get(item).unwrap().clone();

        answer.append(&mut code);
    }

    (tree, answer)
}
