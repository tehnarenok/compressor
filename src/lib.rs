pub mod args;
pub mod bwt;
pub mod constants;
pub mod entropy;
pub mod file;
pub mod huffman;
pub mod mtf;

fn bits_to_byte(bits: &[bool]) -> u8 {
    let mut filled_bits = vec![false; 8];

    for index in 0..bits.len() {
        filled_bits[index] = bits[index];
    }

    filled_bits
        .iter()
        .fold(0, |result, &bit| (result << 1) ^ if bit { 1 } else { 0 })
}

fn byte_to_bits(byte: u8) -> Vec<bool> {
    let mut bits: Vec<bool> = vec![];

    for i in 0..8 {
        let mask = 1 << i;
        bits.push((mask & byte) > 0);
    }

    bits.reverse();

    bits
}

pub fn convert_to_bytes(bits: Vec<bool>) -> Vec<u8> {
    let bits: Vec<&[bool]> = bits.chunks(8).collect();

    bits.iter().map(|bits| bits_to_byte(bits)).collect()
}

pub fn convert_to_bits(bytes: Vec<u8>) -> Vec<bool> {
    let mut bits = vec![];

    for byte in bytes {
        bits.append(&mut byte_to_bits(byte));
    }

    bits
}
