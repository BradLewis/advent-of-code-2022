use blueprint::Blueprint;
use processor::{Processor, State};
use robots::ResourceType;

pub mod blueprint;
pub mod processor;
pub mod robots;

pub fn part1(s: &str) -> usize {
    s.lines()
        .enumerate()
        .map(|(i, l)| {
            let b = Blueprint::create(l);
            let mut state = State::new();
            let mut p = Processor::new(b, 24);

            println!("Start processing for blueprint {}", i + 1);
            let result = p.process_turn(&mut state);
            (i + 1) * result.resources[&ResourceType::Geode]
        })
        .sum()
}

pub fn part2(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let b = Blueprint::create(l);
            let mut state = State::new();
            let mut p = Processor::new(b, 32);

            println!("Start processing for blueprint",);
            let result = p.process_turn(&mut state);
            result.resources[&ResourceType::Geode]
        })
        .product()
}
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let s = fs::read_to_string("test_input.txt").expect("File not found!");
        let result = part1(&s);

        assert_eq!(result, 33);
    }

    #[test]
    fn test_part2() {
        let s = fs::read_to_string("test_input.txt").expect("File not found!");
        let result = part2(&s);

        assert_eq!(result, 32);
    }
}
