use super::tree::TreeNode;

pub fn decode(tree: TreeNode, data: &mut Vec<bool>) -> Vec<usize> {
    let mut node: TreeNode = tree.clone();
    let mut answer: Vec<usize> = vec![];

    while data.len() > 0 {
        if let Some(value) = node.value {
            answer.push(value);

            node = tree.clone();
        }

        let bit = data.pop().unwrap();

        if bit {
            let new_node = node.children[0].borrow().to_owned();
            node = new_node;
        } else {
            let new_node = node.children[1].borrow().to_owned();
            node = new_node;
        }
    }

    if let Some(value) = node.value {
        answer.push(value);
    }

    answer
}
