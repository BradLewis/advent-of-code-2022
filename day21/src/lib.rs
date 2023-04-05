use std::collections::HashMap;

use monkey::Monkey;

mod monkey;

fn parse(s: &str) -> Vec<Monkey> {
    let monkey_map: HashMap<&str, usize> = s
        .lines()
        .enumerate()
        .map(|(i, v)| (v.split_once(':').unwrap().0, 1))
        .collect();

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parse() {
        let input = concat!("root: abcd + efgh", "abcd: 10", "efgh: 35");

        let result = parse(input);
        assert_eq!(
            result,
            vec![
                Monkey::Expression(0, 1, monkey::Operator::Add),
                Monkey::Value(10),
                Monkey::Value(35)
            ]
        );
    }
}
