use std::collections::HashMap;

use crate::robots::{ResourceType, Robot};

#[derive(Debug)]
pub struct Blueprint {
    pub robots: HashMap<ResourceType, Robot>,
    pub max_costs: HashMap<ResourceType, usize>,
}

impl Blueprint {
    pub fn create(s: &str) -> Self {
        let (_, stripped) = s.split_once(':').unwrap();
        let mut max_costs: HashMap<ResourceType, usize> = HashMap::new();
        let robots: HashMap<_, _> = stripped
            .split(". ")
            .map(|s| {
                let r = Robot::new(s);
                for p in r.cost.iter() {
                    let current = max_costs.get(&p.resource_type).unwrap_or(&0);
                    if current < &p.amount {
                        max_costs.insert(p.resource_type, p.amount);
                    }
                }
                (r.resource_collected, r)
            })
            .collect();
        Self { robots, max_costs }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::robots::ResourceType;

    use super::Blueprint;

    #[test]
    fn test_blueprint_creation() {
        let s = String::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let blueprint = Blueprint::create(&s);

        assert_eq!(blueprint.robots.len(), 4);
    }

    #[test]
    fn test_max_costs() {
        let s = String::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let blueprint = Blueprint::create(&s);

        assert_eq!(
            blueprint.max_costs,
            HashMap::from([
                (ResourceType::Ore, 4),
                (ResourceType::Clay, 14),
                (ResourceType::Obsidian, 7)
            ])
        );
    }
}
