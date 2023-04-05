use std::{collections::HashMap, fs};

use monkey::{Monkey, Operation};

mod monkey;

fn parse(s: &str) -> HashMap<&str, usize> {
    let monkey_map: HashMap<&str, usize> = s
        .lines()
        .enumerate()
        .map(|(i, v)| (v.split_once(':').unwrap().0, i))
        .collect();
    monkey_map
}

fn get_monkey_list(s: &str, monkey_map: &HashMap<&str, usize>) -> Vec<Monkey> {
    s.lines()
        .map(|l| {
            let (_, e) = l.split_once(": ").unwrap();
            match e.parse::<isize>() {
                Ok(v) => Monkey::Value(v),
                Err(_) => {
                    let mut splits = e.splitn(3, ' ');
                    let i1 = monkey_map[splits.next().unwrap()];
                    let operation = Operation::from_str(splits.next().unwrap());
                    let i2 = monkey_map[splits.next().unwrap()];
                    Monkey::Expression(i1, i2, operation)
                }
            }
        })
        .collect()
}

pub fn part1(s: &str) -> isize {
    let monkey_map = parse(s);

    let monkies = get_monkey_list(s, &monkey_map);

    let root_index = monkey_map[&"root"];
    monkies[root_index].get_value(&monkies)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input_parse() {
        let input = concat!("root: abcd + efgh\n", "abcd: 10\n", "efgh: 35");

        let result = parse(input);
        assert_eq!(
            result,
            HashMap::from([("root", 0), ("abcd", 1), ("efgh", 2)])
        );
    }

    #[test]
    fn test_get_monkey_list() {
        let input = concat!("root: abcd + efgh\n", "abcd: 10\n", "efgh: 35");

        let monkey_map = parse(input);
        let result = get_monkey_list(input, &monkey_map);
        assert_eq!(
            result,
            vec![
                Monkey::Expression(1, 2, monkey::Operation::Add),
                Monkey::Value(10),
                Monkey::Value(35)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);
        assert_eq!(result, 152);
    }
}
