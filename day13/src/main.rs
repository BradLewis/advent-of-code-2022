use std::cmp::Ordering;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s);
    println!("{}", result);
}

fn part1(s: &str) -> u32 {
    let results: Vec<bool> = s
        .replace("10", "A")
        .split("\n\n")
        .map(|l| {
            let (l1, l2) = l.split_once("\n").unwrap();
            compare(l1.as_bytes(), l2.as_bytes())
        })
        .collect();

    let mut result = 0;
    for i in 0..results.len() {
        if results[i] {
            result += (i as u32) + 1;
        }
    }
    result
}

fn compare(left: &[u8], right: &[u8]) -> bool {
    match (left[0], right[0]) {
        (a, b) if a == b => compare(&left[1..], &right[1..]),
        (_, b']') => false,
        (b']', _) => true,
        (b'[', _) => {
            let subright = [&[right[0], b']'], &right[1..]].concat();
            compare(&left[1..], &subright)
        }
        (_, b'[') => {
            let subleft = [&[left[0], b']'], &left[1..]].concat();
            compare(&subleft, &right[1..])
        }
        (_, _) => match left[0].cmp(&right[0]) {
            Ordering::Less => true,
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare_lines() -> Result<(), String> {
        let l1 = "[[1],[2,3,4]]".as_bytes();
        let l2 = "[[1],4]".as_bytes();
        assert_eq!(compare(&l1, &l2), true);

        let l1 = "[[4,4],4,4]".as_bytes();
        let l2 = "[[4,4],4,4,4]".as_bytes();
        assert_eq!(compare(&l1, &l2), true);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);
        assert_eq!(result, 13);
        Ok(())
    }
}
