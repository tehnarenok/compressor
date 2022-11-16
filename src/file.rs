use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::vec;

use crate::bwt::bwt_reverse;
use crate::constants::BWT_SIZE;
use crate::huffman::decode::decode;
use crate::huffman::tree::TreeNode;
use crate::mtf::mtf_reverse;
use crate::{bwt::bwt, convert_to_bits, convert_to_bytes, huffman::code::code, mtf::mtf};

pub fn compress(filename: &str, out: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let input: Vec<usize> = buffer.iter().map(|i| *i as usize).collect();

    let input: Vec<&[usize]> = input.chunks(BWT_SIZE).collect();

    let bwt_results: Vec<(Vec<usize>, usize)> =
        input.iter().map(|input| bwt(input.to_vec())).collect();

    let mut data: Vec<usize> = vec![];

    for result in &bwt_results {
        data.append(&mut result.0.clone());
    }

    let input = mtf(data);
    let coded = code(&input);
    let serialized = coded.0.serialize();

    write_compressed_to_file(
        serialized,
        bwt_results.iter().map(|result| result.1).collect(),
        coded.1,
        out,
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

fn write_compressed_to_file(
    tree: Vec<bool>,
    bwt_metadata: Vec<usize>,
    data: Vec<bool>,
    file_name: &str,
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

    bits.append(&mut data.clone());

    let bytes = convert_to_bytes(bits);

    let mut f = File::create(file_name).unwrap();

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

fn read_tree(len: usize, data: &mut Vec<bool>) -> TreeNode {
    let mut bits: Vec<bool> = vec![];

    for _ in 0..len {
        bits.push(data.pop().unwrap());
    }

    TreeNode::deserialize(bits)
}

fn read_bwt_metadata(data: &mut Vec<bool>) -> usize {
    read_usize(data)
}

fn read_compressed(data: &mut Vec<bool>) -> Vec<usize> {
    data.reverse();

    let tree_len = read_usize(data);

    let tree = read_tree(tree_len, data);

    let bwt_len = read_usize(data);

    let mut bwt_metadata: Vec<usize> = vec![];

    for _ in 0..bwt_len {
        bwt_metadata.push(read_bwt_metadata(data));
    }

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
