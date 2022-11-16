use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use crate::constants::ALPHABET_LENGTH;

fn h_x(data: &Vec<usize>) -> f32 {
    let mut p: HashMap<usize, usize> = HashMap::new();

    for item in data {
        *p.entry(*item).or_insert(0) += 1;
    }

    let mut entropy = 0.0;

    let len = data.len() as f32;

    for entry in p {
        entropy += (-1.0 * entry.1 as f32 / len) * (entry.1 as f32 / len).log2()
    }

    entropy
}

pub fn h_x_x(data: &Vec<usize>) -> f32 {
    let mut p: HashMap<usize, usize> = HashMap::new();
    let mut p_a_b: Vec<Vec<f32>> = vec![vec![0.; ALPHABET_LENGTH]; ALPHABET_LENGTH];

    for index in 1..data.len() {
        *p.entry(data[index]).or_insert(0) += 1;

        if index > 0 {
            p_a_b[data[index]][data[index - 1]] += 1.;
        }
    }

    for i in p.keys() {
        for j in p.keys() {
            p_a_b[*i][*j] /= *p.get(i).unwrap() as f32;
        }
    }

    let mut entropy = 0.0;
    let len = data.len() as f32;

    for i in p.keys() {
        for j in p.keys() {
            if p_a_b[*i][*j] != 0. {
                entropy += p_a_b[*i][*j] * *p.get(i).unwrap() as f32 * p_a_b[*i][*j].log2() / len;
            }
        }
    }

    -1. * entropy
}

pub fn h_x_xx(data: &Vec<usize>) -> f32 {
    let mut p: HashMap<usize, usize> = HashMap::new();
    let mut p_a_b_c: Vec<Vec<Vec<f32>>> =
        vec![vec![vec![0.; ALPHABET_LENGTH]; ALPHABET_LENGTH]; ALPHABET_LENGTH];

    for index in 2..data.len() {
        *p.entry(data[index]).or_insert(0) += 1;

        p_a_b_c[data[index]][data[index - 1]][data[index - 2]] += 1.;
    }

    let len = data.len() as f32;

    for i in p.keys() {
        for j in p.keys() {
            for z in p.keys() {
                if p_a_b_c[*i][*j][*z] > 0. {
                    p_a_b_c[*i][*j][*z] /= *p.get(i).unwrap() as f32;
                }
            }
        }
    }

    let mut entropy = 0.0;

    for i in p.keys() {
        for j in p.keys() {
            for z in p.keys() {
                if p_a_b_c[*i][*j][*z] != 0. {
                    entropy += p_a_b_c[*i][*j][*z]
                        * *p.get(i).unwrap() as f32
                        * *p.get(j).unwrap() as f32
                        * p_a_b_c[*i][*j][*z].log2()
                        / (len * len);
                }
            }
        }
    }

    -1. * entropy
}

pub fn entropy(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let buffer = buffer.iter().map(|i| *i as usize).collect();

    println!("H(X)    = {:.2}", h_x(&buffer));
    println!("H(X|X)  = {:.2}", h_x_x(&buffer));
    println!("H(X|XX) = {:.2}", h_x_xx(&buffer));
}
