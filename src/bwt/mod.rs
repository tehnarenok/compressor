use crate::constants::ALPHABET_LENGTH;

pub fn bwt(input: Vec<usize>) -> (Vec<usize>, usize) {
    let mut data = input.clone();

    let mut shifts: Vec<Vec<usize>> = vec![];

    shifts.push(data.clone());

    for _ in 1..data.len() {
        data.rotate_left(1);
        shifts.push(data.clone());
    }

    shifts.sort();

    let mut index = None;
    let mut data: Vec<usize> = vec![];

    for i in 0..shifts.len() {
        data.push(*shifts[i].last().unwrap());

        if &input == &shifts[i] {
            index = Some(i);
        }
    }

    (data, index.unwrap())
}

pub fn bwt_reverse(input: (Vec<usize>, usize)) -> Vec<usize> {
    let n = input.0.len();
    let data = input.0;

    let mut count: Vec<usize> = vec![0; ALPHABET_LENGTH];

    for el in &data {
        count[*el] += 1;
    }

    let mut sum = 0;

    for i in 0..ALPHABET_LENGTH {
        sum = sum + count[i];

        count[i] = sum - count[i];
    }

    let mut t: Vec<usize> = vec![0; n];

    for i in 0..n {
        t[count[data[i]]] = i;
        count[data[i]] += 1;
    }

    let mut j = t[input.1];

    let mut answer = vec![0; n];

    for i in 0..n {
        answer[i] = data[j];
        j = t[j];
    }

    answer
}
