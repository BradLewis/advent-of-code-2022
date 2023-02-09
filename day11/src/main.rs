use std::fs;
type Operation = Box<dyn Fn(i32) -> i32>;

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    true_target: i32,
    false_target: i32,
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

fn parse_to_int(s: &str, prefix: &str) -> i32 {
    s.trim().strip_prefix(prefix).unwrap().parse().unwrap()
}

fn create_monkies(s: &str) -> Vec<Monkey> {
    s.split("\n\n").map(|m| Monkey::new(m)).collect()
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    part1(&s);
}

fn part1(s: &str) {
    // let monkies = s.split("\n\n").map(|m|)
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
            parse_to_int("Test: divisible by 19", "Test: divisible by "),
            19
        );
        assert_eq!(
            parse_to_int("If true: throw to monkey 2", "If true: throw to monkey "),
            2
        );
        assert_eq!(
            parse_to_int("If false: throw to monkey 7", "If false: throw to monkey "),
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
}
