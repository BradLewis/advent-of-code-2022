use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(ResourceType::Ore),
            "clay" => Ok(ResourceType::Clay),
            "obsidian" => Ok(ResourceType::Obsidian),
            "geode" => Ok(ResourceType::Geode),
            _ => Err(format!("'{}' is not a valid ResourceType", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Price {
    pub resource_type: ResourceType,
    pub amount: usize,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub cost: Vec<Price>,
    pub resource_collected: ResourceType,
}

impl Robot {
    pub fn new(s: &str) -> Self {
        let values = parse(s, r"(ore|obsidian|clay|geode|\d+)");
        let resource_collected = ResourceType::from_str(&values[0]).unwrap();
        let mut cost: Vec<Price> = Vec::new();
        for i in (1..values.len()).step_by(2) {
            cost.push(Price {
                amount: values[i].parse::<usize>().unwrap(),
                resource_type: ResourceType::from_str(&values[i + 1]).unwrap(),
            })
        }
        Self {
            cost,
            resource_collected,
        }
    }

    pub fn can_afford(&self, resources: &HashMap<ResourceType, usize>) -> bool {
        for price in self.cost.iter() {
            if resources.get(&price.resource_type).unwrap_or(&0) < &price.amount {
                return false;
            }
        }
        true
    }
}

fn parse(s: &str, re: &str) -> Vec<String> {
    let regex: Regex = Regex::new(re).unwrap();
    regex
        .find_iter(s)
        .map(|m| String::from_str(m.as_str()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::robots::parse;

    macro_rules! str {
        ($a:expr) => {
            String::from($a)
        };
    }

    #[test]
    fn test_regex() {
        let result = parse(
            "Each obsidian robot costs 3 ore and 8 clay.",
            r"(ore|obsidian|clay|geode|\d+)",
        );

        assert_eq!(result.len(), 5);
        assert_eq!(
            result,
            vec![
                str!("obsidian"),
                str!("3"),
                str!("ore"),
                str!("8"),
                str!("clay")
            ]
        );
    }

    mod robots {
        use std::collections::HashMap;

        use crate::robots::{ResourceType, Robot};

        #[test]
        fn test_create_one_cost() {
            let s = String::from("Each ore robot costs 4 ore.");
            let robot = Robot::new(&s);

            assert_eq!(robot.resource_collected, ResourceType::Ore);
            assert_eq!(robot.cost.len(), 1);
            assert_eq!(robot.cost[0].amount, 4);
            assert_eq!(robot.cost[0].resource_type, ResourceType::Ore);
        }

        #[test]
        fn test_create_two_cost() {
            let s = String::from("Each obsidian robot costs 3 ore and 8 clay.");
            let robot = Robot::new(&s);

            assert_eq!(robot.resource_collected, ResourceType::Obsidian);
            assert_eq!(robot.cost.len(), 2);
            assert_eq!(robot.cost[0].amount, 3);
            assert_eq!(robot.cost[0].resource_type, ResourceType::Ore);
            assert_eq!(robot.cost[1].amount, 8);
            assert_eq!(robot.cost[1].resource_type, ResourceType::Clay);
        }

        #[test]
        fn test_can_afford() {
            let s = String::from("Each obsidian robot costs 3 ore and 8 clay.");
            let robot = Robot::new(&s);

            assert!(!robot.can_afford(&HashMap::new()));
            assert!(robot.can_afford(&HashMap::from([
                (ResourceType::Ore, 3),
                (ResourceType::Clay, 8),
                (ResourceType::Obsidian, 0),
                (ResourceType::Geode, 0),
            ])))
        }
    }
}
