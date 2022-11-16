use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{convert_to_bits, convert_to_bytes};

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub value: Option<usize>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: Option<usize>) -> Self {
        Self {
            value,
            children: vec![],
            parent: None,
        }
    }

    pub fn push(&mut self, child: Rc<RefCell<TreeNode>>) {
        self.children.push(child);
    }

    fn get_codes(&self, code: Vec<bool>, map: &mut HashMap<usize, Vec<bool>>) {
        if let Some(value) = self.value {
            map.insert(value, code.clone());
        }

        for index in 0..self.children.len().min(2) {
            let mut new_code = code.clone();
            new_code.push(index % 2 == 0);
            self.children[index].borrow().get_codes(new_code, map);
        }
    }

    pub fn codes(&self) -> HashMap<usize, Vec<bool>> {
        let mut map = HashMap::new();

        self.get_codes(vec![], &mut map);

        map
    }
}

impl TreeNode {
    fn encode_node(&self, bits: &mut Vec<bool>) {
        if let Some(value) = self.value {
            bits.push(true);
            convert_to_bits(value.to_ne_bytes().to_vec()).len();
            bits.append(&mut convert_to_bits(value.to_ne_bytes()[..1].to_vec()));
        } else {
            bits.push(false);

            for index in 0..self.children.len().min(2) {
                self.children[index].borrow().encode_node(bits);
            }
        }
    }

    pub fn serialize(&self) -> Vec<bool> {
        let mut bits: Vec<bool> = vec![];

        self.encode_node(&mut bits);

        bits
    }

    fn read_usize(data: &mut Vec<bool>) -> usize {
        let mut bits = vec![];
        for _ in 0..8 {
            bits.push(data.pop().unwrap())
        }

        let bytes = convert_to_bytes(bits);

        usize::from_ne_bytes([bytes[0], 0, 0, 0, 0, 0, 0, 0])
    }

    fn read_node(data: &mut Vec<bool>) -> Self {
        if data.pop().unwrap() {
            Self::new(Some(Self::read_usize(data)))
        } else {
            let left = Rc::new(RefCell::new(Self::read_node(data)));
            let right = Rc::new(RefCell::new(Self::read_node(data)));

            let mut node = Self::new(None);

            node.push(left);
            node.push(right);

            node
        }
    }

    pub fn deserialize(data: Vec<bool>) -> Self {
        let mut data = data.clone();
        data.reverse();

        Self::read_node(&mut data)
    }
}
