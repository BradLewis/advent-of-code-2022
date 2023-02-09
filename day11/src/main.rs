use std::{fs, str::FromStr};

type Operation = Box<dyn Fn(i32) -> i32>;

#[derive(Debug, PartialEq, Eq)]
struct Throw {
    target: usize,
    item: i32,
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn new(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next();
        Self {
            items: parse_items(lines.next().unwrap()),
            operation: parse_operation(lines.next().unwrap()),
            test: parse_to_int(lines.next().unwrap(), "Test: divisible by "),
            true_target: parse_to_int(lines.next().unwrap(), "If true: throw to monkey "),
            false_target: parse_to_int(lines.next().unwrap(), "If false: throw to monkey "),
        }
    }

    fn throw(&mut self) -> Vec<Throw> {
        let mut throws: Vec<Throw> = Vec::new();
        for item in self.items.iter() {
            let new_item = (self.operation)(*item) / 3;
            let target = match new_item % self.test {
                0 => self.true_target,
                _ => self.false_target,
            };
            throws.push(Throw {
                item: new_item,
                target,
            })
        }
        self.items = Vec::new();
        throws
    }
}

fn parse_items(s: &str) -> Vec<i32> {
    let prefix = "Starting items: ";

    s.trim()
        .strip_prefix(prefix)
        .unwrap()
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_operation(s: &str) -> Operation {
    let prefix = "Operation: new = ";
    let operands: Vec<&str> = s.trim().strip_prefix(prefix).unwrap().split(" ").collect();
    match operands[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),
        ["old", "*", y] => {
            let y = y.parse::<i32>().unwrap();
            Box::new(move |x| x * y)
        }
        ["old", "+", y] => {
            let y = y.parse::<i32>().unwrap();
            Box::new(move |x| x + y)
        }
        _ => unreachable!(),
    }
}

fn parse_to_int<T>(s: &str, prefix: &str) -> T
where
    T: FromStr,
{
    s.trim()
        .strip_prefix(prefix)
        .unwrap()
        .parse::<T>()
        .ok()
        .unwrap()
}

fn create_monkies(s: &str) -> Vec<Monkey> {
    s.split("\n\n").map(|m| Monkey::new(m)).collect()
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let mut p1 = Part1::new(&s);
    let result = p1.run(20);
    println!("{}", result);
}

struct Part1 {
    monkies: Vec<Monkey>,
    counts: Vec<usize>,
}

impl Part1 {
    fn new(s: &str) -> Self {
        let monkies = create_monkies(s);
        let len = monkies.len();
        Self {
            monkies,
            counts: vec![0; len],
        }
    }

    fn run(&mut self, iterations: i32) -> usize {
        for _ in 0..iterations {
            self.run_cycle();
        }

        let mut final_counts = self.counts.clone();
        final_counts.sort_by(|a, b| b.cmp(a));
        final_counts[0] * final_counts[1]
    }

    fn run_cycle(&mut self) {
        for i in 0..self.monkies.len() {
            let throws = self.monkies[i].throw();
            self.counts[i] += throws.len();
            self.process_throws(throws);
        }
    }
    fn process_throws(&mut self, throws: Vec<Throw>) {
        for throw in throws.iter() {
            self.process_throw(throw);
        }
    }

    fn process_throw(&mut self, throw: &Throw) {
        self.monkies[throw.target].items.push(throw.item);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_items() -> Result<(), String> {
        let s = "Starting items: 75, 64";
        let result = parse_items(s);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 75);
        assert_eq!(result[1], 64);
        Ok(())
    }

    #[test]
    fn test_parse_operation() -> Result<(), String> {
        let s = "Operation: new = old * 13";
        let result = parse_operation(s);
        assert_eq!(result(2), 26);

        let s = "Operation: new = old * old";
        let result = parse_operation(s);
        assert_eq!(result(2), 4);

        let s = "Operation: new = old + 13";
        let result = parse_operation(s);
        assert_eq!(result(2), 15);
        Ok(())
    }

    #[test]
    fn test_parse_to_int() -> Result<(), String> {
        assert_eq!(
            parse_to_int::<i32>("Test: divisible by 19", "Test: divisible by "),
            19
        );
        assert_eq!(
            parse_to_int::<usize>("If true: throw to monkey 2", "If true: throw to monkey "),
            2
        );
        assert_eq!(
            parse_to_int::<usize>("If false: throw to monkey 7", "If false: throw to monkey "),
            7
        );
        Ok(())
    }

    #[test]
    fn test_create_monkey() -> Result<(), String> {
        let s = "Monkey 1:
          Starting items: 50, 99, 80, 84, 65, 95
          Operation: new = old + 2
          Test: divisible by 3
            If true: throw to monkey 4
            If false: throw to monkey 5";

        let monkey = Monkey::new(s);

        assert_eq!(monkey.items.len(), 6);
        assert_eq!((monkey.operation)(3), 5);
        assert_eq!(monkey.test, 3);
        assert_eq!(monkey.true_target, 4);
        assert_eq!(monkey.false_target, 5);
        Ok(())
    }

    #[test]
    fn test_create_monkies() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let monkies = create_monkies(&s);
        assert_eq!(monkies.len(), 4);
        Ok(())
    }

    #[test]
    fn test_monkey_throw() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut monkies = create_monkies(&s);

        let throws = monkies[0].throw();

        assert_eq!(throws.len(), 2);
        assert_eq!(monkies[0].items.len(), 0);
        assert_eq!(
            throws[0],
            Throw {
                target: 3,
                item: 500
            }
        );

        let throws = monkies[2].throw();
        assert_eq!(
            throws[0],
            Throw {
                target: 1,
                item: 2080
            }
        );
        Ok(())
    }

    #[test]
    fn test_part1_process_throw() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut part1 = Part1::new(&s);
        let throws = part1.monkies[0].throw();
        part1.process_throw(&throws[0]);

        assert_eq!(part1.monkies[3].items.len(), 2);
        assert_eq!(*part1.monkies[3].items.last().unwrap(), 500);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut part1 = Part1::new(&s);
        let result = part1.run(20);

        assert_eq!(part1.counts, vec![101, 95, 7, 105]);
        assert_eq!(result, 10605);
        Ok(())
    }
}
