pub mod build_tree;
pub mod code;
pub mod decode;
pub mod tree;

// #[derive(Clone, Copy, Debug)]
// pub struct Item {
//     pub element: Option<usize>,
//     pub parent: usize,
//     pub weight: usize,
// }

// #[derive(Clone, Debug)]
// pub struct Tree {
//     pub data: Vec<Item>,
// }

// impl Tree {
//     pub fn serialize(&self) -> Vec<u8> {
//         let mut data = vec![];

//         data.push(self.data.len().to_ne_bytes()[0]);

//         for item in &self.data {
//             data.push(if let Some(element) = item.element {
//                 element.to_ne_bytes()[0]
//             } else {
//                 [-1]
//             });
//             data.push(item.parent.to_ne_bytes()[0]);
//             data.push(item.weight.to_ne_bytes()[0]);
//         }

//         data
//     }

//     pub fn deserialize(mut data: Vec<u8>) -> Self {
//         let len: Vec<u8> = data.drain(..1).collect();

//         let len = usize::from_ne_bytes([len[0], 0, 0, 0, 0, 0, 0, 0]);

//         let mut items: Vec<Item> = vec![];

//         for _ in 0..len {
//             let item: Vec<u8> = data.drain(..3).collect();

//             items.push(Item {
//                 element: usize::from_ne_bytes([item[0], 0, 0, 0, 0, 0, 0, 0]),
//                 parent: usize::from_ne_bytes([item[1], 0, 0, 0, 0, 0, 0, 0]),
//                 weight: usize::from_ne_bytes([item[2], 0, 0, 0, 0, 0, 0, 0]),
//             })
//         }

//         Self { data: items }
//     }
// }

// impl Tree {
//     pub fn new(item: Item) -> Self {
//         Self {
//             data: vec![Item {
//                 element: item.element,
//                 weight: item.weight,
//                 parent: 0,
//             }],
//         }
//     }

//     fn root(&self) -> Option<usize> {
//         for index in 0..self.data.len() {
//             if self.data[index].parent == index {
//                 return Some(index);
//             }
//         }

//         None
//     }

//     pub fn push(&mut self, element: usize, parent: usize) -> usize {
//         let weight: usize = if let Some(_) = self.data.iter().position(|el| el.parent == parent) {
//             1
//         } else {
//             0
//         };

//         self.data.push(Item {
//             element,
//             parent,
//             weight,
//         });

//         self.data.len() - 1
//     }

//     pub fn append(&mut self, node: usize, tree: Tree) {
//         let root = tree.root().unwrap();

//         let mut to_insert: Vec<(usize, usize)> = vec![(root, node)];

//         while to_insert.len() > 0 {
//             let item_to_insert = dbg!(to_insert.swap_remove(0));

//             let inserted_index = self.push(tree.data[item_to_insert.0].element, item_to_insert.1);

//             for index in 0..tree.data.len() {
//                 if item_to_insert.0 == tree.data[index].parent && index != item_to_insert.0 {
//                     to_insert.push((index, inserted_index));
//                 }
//             }

//             dbg!(&to_insert);
//         }
//     }
// }
