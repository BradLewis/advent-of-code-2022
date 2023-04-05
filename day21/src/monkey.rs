#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn evaluate(&self, v1: isize, v2: isize) -> isize {
        match self {
            Self::Add => v1 + v2,
            Self::Subtract => v1 - v2,
            Self::Multiply => v1 * v2,
            Self::Divide => v1 / v2,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Monkey {
    Value(isize),
    Expression(usize, usize, Operator),
}

impl Monkey {
    pub fn get_value(&self, monkies: &[Monkey]) -> isize {
        match self {
            Monkey::Value(v) => *v,
            Monkey::Expression(v1, v2, operator) => {
                let v1 = monkies[*v1].get_value(monkies);
                let v2 = monkies[*v2].get_value(monkies);
                operator.evaluate(v1, v2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_get_value() {
        let monkies = vec![
            Monkey::Value(2),
            Monkey::Value(5),
            Monkey::Expression(0, 1, Operator::Add),
        ];
        let monkey = &monkies[2];
        let result = monkey.get_value(&monkies);
        assert_eq!(result, 7)
    }
}
