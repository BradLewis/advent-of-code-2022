use std::cmp::Ordering;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s);
    println!("{}", result);

    let result = part2(&s);
    println!("{}", result);
}

fn part1(s: &str) -> u32 {
    let results: Vec<bool> = s
        .replace("10", "A")
        .split("\n\n")
        .map(|l| {
            let (l1, l2) = l.split_once("\n").unwrap();
            compare(l1.as_bytes(), l2.as_bytes()) == Ordering::Less
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

fn part2(s: &str) -> usize {
    let s2 = "[[2]]";
    let s6 = "[[6]]";
    let mut result = s.replace("10", "A").replace("\n\n", "\n");
    result.push_str(&(String::from("\n") + s2));
    result.push_str(&(String::from("\n") + s6));
    let mut r: Vec<_> = result.lines().collect();
    r.sort_by(|l1, l2| compare(l1.as_bytes(), l2.as_bytes()));
    (r.iter().position(|&e| e == s6).unwrap() + 1) * (r.iter().position(|&e| e == s2).unwrap() + 1)
}

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    match (left[0], right[0]) {
        (a, b) if a == b => compare(&left[1..], &right[1..]),
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => {
            let subright = [&[right[0], b']'], &right[1..]].concat();
            compare(&left[1..], &subright)
        }
        (_, b'[') => {
            let subleft = [&[left[0], b']'], &left[1..]].concat();
            compare(&subleft, &right[1..])
        }
        (_, _) => left[0].cmp(&right[0]),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare_lines() -> Result<(), String> {
        let l1 = "[[1],[2,3,4]]".as_bytes();
        let l2 = "[[1],4]".as_bytes();
        assert_eq!(compare(&l1, &l2), Ordering::Less);

        let l1 = "[[4,4],4,4]".as_bytes();
        let l2 = "[[4,4],4,4,4]".as_bytes();
        assert_eq!(compare(&l1, &l2), Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part2(&s);
        assert_eq!(result, 140);
        Ok(())
    }
}
