use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let total: u32 = content
        .lines()
        .map(|m| {
            let half = m.len() / 2;
            let (s1, s2) = m.split_at(half);
            let common = get_common_element(s1, s2);
            get_value(common)
        })
        .sum();

    println!("{}", total)
}

fn part2(content: &str) {
    let lines = content.lines();
    let mut total = 0;
    for (s1, s2, s3) in lines.tuples() {
        let common = get_common_element_three(s1, s2, s3);
        total += get_value(common);
    }

    println!("{}", total);
}

fn get_common_element(s1: &str, s2: &str) -> char {
    let mut temp = HashMap::new();

    for c in s1.chars() {
        temp.insert(c, c);
    }

    for c in s2.chars() {
        if temp.contains_key(&c) {
            return c;
        }
    }
    unreachable!()
}

fn get_common_element_three(s1: &str, s2: &str, s3: &str) -> char {
    let mut temp1 = HashMap::new();
    let mut temp2 = HashMap::new();

    for c in s1.chars() {
        temp1.insert(c, c);
    }

    for c in s2.chars() {
        temp2.insert(c, c);
    }

    for c in s3.chars() {
        if temp1.contains_key(&c) {
            if temp2.contains_key(&c) {
                return c;
            }
        }
    }
    unreachable!()
}

fn get_value(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    }
    return c as u32 - 38;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common_element() -> Result<(), String> {
        assert_eq!(get_common_element("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
        assert_eq!(get_common_element("PmmdzqPrV", "vPwwTWBwg"), 'P');
        Ok(())
    }

    #[test]
    fn test_get_common_element_three() -> Result<(), String> {
        assert_eq!(
            get_common_element_three(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            'r'
        );
        assert_eq!(
            get_common_element_three(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            'Z'
        );
        Ok(())
    }

    #[test]
    fn test_get_value() -> Result<(), String> {
        assert_eq!(get_value('a'), 1);
        assert_eq!(get_value('A'), 27);
        assert_eq!(get_value('L'), 38);
        Ok(())
    }
}
