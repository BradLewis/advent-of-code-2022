#[derive(Debug, PartialEq, Eq)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
pub struct Price {
    pub resource_type: ResourceType,
    pub amount: usize,
}

#[derive(Debug)]
pub struct Robot {
    pub cost: Vec<Price>,
    pub resource_collected: ResourceType,
}

impl Robot {
    pub fn new(s: &str) -> Self {
        let cost = vec![Price {
            amount: 0,
            resource_type: ResourceType::Ore,
        }];
        let resource_collected = ResourceType::Ore;
        Self {
            cost,
            resource_collected,
        }
    }
}

mod tests {
    use crate::robots::{ResourceType, Robot};

    #[test]
    fn test_create_robot() {
        let s = String::from("Each ore robot costs 4 ore.");
        let robot = Robot::new(&s);

        assert_eq!(robot.resource_collected, ResourceType::Ore);
        assert_eq!(robot.cost.len(), 1);
        assert_eq!(robot.cost[0].amount, 4);
        assert_eq!(robot.cost[0].resource_type, ResourceType::Ore);
    }
}
