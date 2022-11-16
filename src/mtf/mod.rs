use crate::constants::ALPHABET_LENGTH;

fn find(array: &Vec<usize>, el: usize) -> usize {
    array.iter().position(|item| *item == el).unwrap()
}

fn get_alphabet() -> Vec<usize> {
    (0..ALPHABET_LENGTH).collect()
}

pub fn mtf(input: Vec<usize>) -> Vec<usize> {
    let mut alphabet = get_alphabet();

    let mut answer: Vec<usize> = vec![];

    for item in input {
        let position = find(&alphabet, item);

        answer.push(position);

        alphabet.remove(position);

        alphabet.insert(0, item);
    }

    answer
}

pub fn mtf_reverse(input: Vec<usize>) -> Vec<usize> {
    let mut alphabet: Vec<usize> = get_alphabet();

    let mut answer: Vec<usize> = vec![];

    for position in input {
        let item = alphabet[position];

        answer.push(item);

        alphabet.remove(position);

        alphabet.insert(0, item);
    }

    answer
}
