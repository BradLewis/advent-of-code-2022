use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    println!("{}", get_marker_character(content, 4));
}

fn part2(content: &str) {
    println!("{}", get_marker_character(content, 14));
}

fn get_marker_character(s: &str, marker_len: usize) -> usize {
    let l = marker_len - 1;
    let mut current: Vec<char> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if i < l {
            current.push(c);
            continue;
        }
        let mut uniques = current.clone();
        uniques.sort();
        uniques.dedup();
        // println!("{i}:{c}:{}", uniques.len());
        if uniques.len() != l {
            current[i % l] = c;
            continue;
        }
        if !current.contains(&c) {
            return i + 1;
        }
        current[i % l] = c;
    }
    !unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_marker() -> Result<(), String> {
        assert_eq!(get_marker_character("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(get_marker_character("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_marker_character("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            get_marker_character("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            get_marker_character("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
        assert_eq!(
            get_marker_character("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
        Ok(())
    }
}
