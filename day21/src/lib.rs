use std::collections::HashMap;

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

fn gradient_descent(
    monkies: &mut [Monkey],
    humn_index: usize,
    r1_index: usize,
    r2_index: usize,
) -> isize {
    let mut prev_error =
        (monkies[r1_index].get_value(monkies) - monkies[r2_index].get_value(monkies)).abs();
    let mut guess: isize = 1;
    let mut prev_guess = monkies[humn_index].get_value(monkies);
    loop {
        monkies[humn_index] = Monkey::Value(guess);
        let v1 = monkies[r1_index].get_value(monkies);
        let v2 = monkies[r2_index].get_value(monkies);
        let e = (v1 - v2).abs();
        if e < 1 {
            break;
        }
        let gradient = match e.cmp(&prev_error) {
            std::cmp::Ordering::Equal => 1.0,
            _ => (guess - prev_guess) as f64 / (e - prev_error) as f64,
        };
        prev_guess = guess;
        prev_error = e;
        guess -= (e as f64 * gradient) as isize;
    }
    guess
}

pub fn part1(s: &str) -> isize {
    let monkey_map = parse(s);

    let monkies = get_monkey_list(s, &monkey_map);

    let root_index = monkey_map[&"root"];
    monkies[root_index].get_value(&monkies)
}

pub fn part2(s: &str) -> isize {
    let monkey_map = parse(s);

    let mut monkies = get_monkey_list(s, &monkey_map);

    let root_index = monkey_map[&"root"];
    let humn_index = monkey_map[&"humn"];

    let (r1_index, r2_index) = match monkies[root_index] {
        Monkey::Value(_) => unreachable!(),
        Monkey::Expression(v1, v2, _) => (v1, v2),
    };
    gradient_descent(&mut monkies, humn_index, r1_index, r2_index)
}

#[cfg(test)]
mod tests {

    use std::fs;

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

    #[test]
    fn test_part2() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part2(&s);
        assert_eq!(result, 301);
    }
}
