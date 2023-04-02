type IndexValue = (usize, isize);

fn get_initial_index(vec: &[IndexValue], initial_index: usize) -> usize {
    vec.iter().position(|&(i, _)| i == initial_index).unwrap()
}

fn index_of(vec: &[IndexValue], value: isize) -> usize {
    vec.iter().position(|&(_, v)| value == v).unwrap()
}

fn load_input(s: &str, key: isize) -> Vec<IndexValue> {
    s.lines()
        .map(|l| l.parse::<isize>().unwrap() * key)
        .enumerate()
        .collect()
}

fn get_result(pos: usize, vec: &[IndexValue], iterations: usize) -> isize {
    vec[(pos + iterations) % vec.len()].1
}

pub fn decode(s: &str, key: isize, num_rounds: usize) -> isize {
    let mut v = load_input(s, key);
    let len = v.len();
    for _ in 0..num_rounds {
        for i in 0..v.len() {
            let index = get_initial_index(&v, i);
            let value = v[index].1;
            let new_position = index as isize + value;
            let new_index = new_position.rem_euclid(len as isize - 1) as usize;

            let moving = v.remove(index);
            v.insert(new_index, moving);
        }
    }

    let zero_position = index_of(&v, 0);
    get_result(zero_position, &v, 1000)
        + get_result(zero_position, &v, 2000)
        + get_result(zero_position, &v, 3000)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{decode, get_initial_index, index_of, IndexValue};

    #[test]
    fn test_part1() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let result = decode(&s, 1, 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_index_of() {
        let v: Vec<IndexValue> = vec![0, 1, 2, -3, 4].into_iter().enumerate().collect();
        assert_eq!(index_of(&v, -3), 3);
    }

    #[test]
    fn test_get_initial_index() {
        let v: Vec<IndexValue> = vec![0, 1, 2, -3, 4].into_iter().enumerate().collect();
        assert_eq!(get_initial_index(&v, 1), 1);
    }

    #[test]
    fn test_part2() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let result = decode(&s, 811_589_153, 10);
        assert_eq!(result, 1_623_178_306);
    }
}
