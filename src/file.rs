use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::vec;

use crate::bwt::bwt_reverse;
use crate::constants::BWT_SIZE;
use crate::huffman::decode::decode;
use crate::huffman::tree::TreeNode;
use crate::mtf::mtf_reverse;
use crate::{bwt::bwt, convert_to_bits, convert_to_bytes, huffman::encode::encode, mtf::mtf};

pub fn compress(filename: &str, out: &str, show_info: bool) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let input: Vec<usize> = buffer.iter().map(|i| *i as usize).collect();

    let input_size = input.len();

    if show_info {
        println!("Source file size (bytes): {}", input.len());
    }

    let input: Vec<&[usize]> = input.chunks(BWT_SIZE).collect();

    let bwt_results: Vec<(Vec<usize>, usize)> =
        input.iter().map(|input| bwt(input.to_vec())).collect();

    let mut data: Vec<usize> = vec![];

    for result in &bwt_results {
        data.append(&mut result.0.clone());
    }

    let input = mtf(data);
    let coded = encode(&input, show_info);
    let serialized = coded.0.serialize();

    write_compressed_to_file(
        serialized,
        bwt_results.iter().map(|result| result.1).collect(),
        coded.1,
        out,
        show_info,
        input_size,
    );
}

pub fn decompress(filename: &str, out: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let bytes = read_compressed(&mut convert_to_bits(buffer));

    let bytes: Vec<u8> = bytes.iter().map(|i| *i as u8).collect();

    let mut f = File::create(out).unwrap();

    f.write_all(bytes.as_slice()).unwrap();
}

pub fn print_info(filename: &str, print_tree: bool) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let mut bits = convert_to_bits(buffer);
    bits.reverse();

    let len = bits.len();

    println!("File size (bytes): {}", bits.len() / 8);

    let tree = read_tree(&mut bits);
    let bwt_len = read_usize(&mut bits);
    let mut bwt_metadata: Vec<usize> = vec![];

    for _ in 0..bwt_len {
        bwt_metadata.push(read_bwt_metadata(&mut bits));
    }

    read_end(&mut bits);

    println!("Metadata size (bytes): {}", (len - bits.len()) / 8);

    if print_tree {
        println!("-----TREE-----");
        println!("Tree size (bits): {}", tree.1 / 8);
        println!("Codes: ");

        for entry in tree.0.codes() {
            let code: Vec<&str> = entry
                .1
                .iter()
                .map(|el| if *el { "1" } else { "0" })
                .collect();

            println!("[{}]: {}", entry.0, code.join(""))
        }

        println!("-------------");
    }
}

fn write_compressed_to_file(
    tree: Vec<bool>,
    bwt_metadata: Vec<usize>,
    data: Vec<bool>,
    file_name: &str,
    show_info: bool,
    input_size: usize,
) {
    let tree_len = tree.len();

    let mut bits: Vec<bool> = vec![];

    bits.append(&mut convert_to_bits(tree_len.to_ne_bytes()[..8].to_vec()));
    bits.append(&mut tree.clone());

    bits.append(&mut convert_to_bits(
        bwt_metadata.len().to_ne_bytes()[..8].to_vec(),
    ));

    for bwt in bwt_metadata {
        bits.append(&mut convert_to_bits(bwt.to_ne_bytes()[..8].to_vec()));
    }

    let end = 8 - (data.len() + tree.len() % 8) % 8;

    bits.append(&mut convert_to_bits(vec![end as u8]));

    if show_info {
        println!("Metadata size (bytes): {}", &bits.len() / 8);
    }

    bits.append(&mut data.clone());

    let bytes = convert_to_bytes(bits.clone());

    assert_eq!(convert_to_bits(bytes.clone())[..bits.len()], bits);

    let mut f = File::create(file_name).unwrap();

    if show_info {
        let size = bytes.len();
        println!("Encoded file size (bytes): {}", size);
        println!(
            "Compressed ratio: {:.2}%",
            (input_size as f32 - size as f32) / input_size as f32 * 100.0
        );
    }

    f.write_all(bytes.as_slice()).unwrap();
}

fn read_usize(data: &mut Vec<bool>) -> usize {
    let mut bits = vec![];
    for _ in 0..8 * 8 {
        bits.push(data.pop().unwrap())
    }

    let bytes = convert_to_bytes(bits);

    usize::from_ne_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

fn read_tree(data: &mut Vec<bool>) -> (TreeNode, usize) {
    let tree_len = read_usize(data);

    let mut bits: Vec<bool> = vec![];

    for _ in 0..tree_len {
        bits.push(data.pop().unwrap());
    }

    (TreeNode::deserialize(bits), tree_len)
}

fn read_bwt_metadata(data: &mut Vec<bool>) -> usize {
    read_usize(data)
}

fn read_end(data: &mut Vec<bool>) -> usize {
    let mut bits: Vec<bool> = vec![];

    for _ in 0..8 {
        bits.push(data.pop().unwrap());
    }

    convert_to_bytes(bits)[0] as usize
}

fn read_compressed(data: &mut Vec<bool>) -> Vec<usize> {
    data.reverse();

    let tree = read_tree(data).0;
    let bwt_len = read_usize(data);
    let mut bwt_metadata: Vec<usize> = vec![];

    for _ in 0..bwt_len {
        bwt_metadata.push(read_bwt_metadata(data));
    }

    let end = read_end(data);

    data.reverse();

    for _ in 0..end {
        data.pop();
    }

    data.reverse();

    let output = mtf_reverse(decode(tree, data));

    let output: Vec<&[usize]> = output.chunks(BWT_SIZE).collect();

    let mut out: Vec<usize> = vec![];

    for index in 0..output.len() {
        out.append(&mut bwt_reverse((
            output[index].to_vec(),
            bwt_metadata[index],
        )));
    }

    out
}
