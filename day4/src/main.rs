use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn from_string(s: &str) -> Range {
        let split_string: Vec<&str> = s.split("-").collect();
        Range {
            min: split_string[0].parse::<u32>().unwrap(),
            max: split_string[1].parse::<u32>().unwrap(),
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let total: u32 = content
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(",").collect();
            let result = check_overlap(split[0], split[1]);
            match result {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("{}", total);
}

fn part2(content: &str) {
    let total: u32 = content
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(",").collect();
            let result = check_overlap_2(split[0], split[1]);
            match result {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("{}", total);
}

fn check_overlap(s1: &str, s2: &str) -> bool {
    let r1 = Range::from_string(s1);
    let r2 = Range::from_string(s2);

    if r1.min <= r2.min && r1.max >= r2.max {
        return true;
    }
    if r1.min >= r2.min && r1.max <= r2.max {
        return true;
    }
    return false;
}

fn check_overlap_2(s1: &str, s2: &str) -> bool {
    let r1 = Range::from_string(s1);
    let r2 = Range::from_string(s2);

    if r1.min >= r2.min && r1.min <= r2.max {
        return true;
    }
    if r1.max >= r2.min && r1.max <= r2.max {
        return true;
    }
    if r2.min >= r1.min && r2.min <= r1.max {
        return true;
    }
    if r2.max >= r1.min && r2.max <= r1.max {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_overlap() -> Result<(), String> {
        assert_eq!(check_overlap("2-4", "6-8"), false);
        assert_eq!(check_overlap("2-8", "3-4"), true);
        assert_eq!(check_overlap("2-8", "3-9"), false);
        assert_eq!(check_overlap("4-6", "6-6"), true);
        Ok(())
    }

    #[test]
    fn test_check_overlap_2() -> Result<(), String> {
        assert_eq!(check_overlap_2("2-4", "6-8"), false);
        // assert_eq!(check_overlap_2("2-8", "3-4"), true);
        // assert_eq!(check_overlap_2("2-8", "3-9"), true);
        // assert_eq!(check_overlap_2("4-6", "6-6"), true);
        Ok(())
    }

    #[test]
    fn test_get_range() -> Result<(), String> {
        assert_eq!(Range::from_string("2-4"), Range { min: 2, max: 4 });
        assert_eq!(Range::from_string("2-8"), Range { min: 2, max: 8 });
        Ok(())
    }
}
