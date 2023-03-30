use crate::{
    blueprint::Blueprint,
    robots::{ResourceType, Robot},
};
use std::collections::HashMap;

macro_rules! init_resource_count {
    () => {
        HashMap::from([
            (ResourceType::Ore, 0),
            (ResourceType::Clay, 0),
            (ResourceType::Obsidian, 0),
            (ResourceType::Geode, 0),
        ])
    };
}

#[derive(Debug, Clone)]
pub struct State {
    pub robots: HashMap<ResourceType, usize>,
    pub resources: HashMap<ResourceType, usize>,
    pub iteration: usize,
}

impl State {
    pub fn new() -> Self {
        let mut robots = init_resource_count!();
        robots.insert(ResourceType::Ore, 1);
        Self {
            robots,
            resources: init_resource_count!(),
            iteration: 0,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Processor {
    pub blueprint: Blueprint,
}

impl Processor {
    pub fn new(blueprint: Blueprint) -> Self {
        Self { blueprint }
    }

    pub fn process_turn(&mut self, state: &mut State) -> usize {
        state.iteration += 1;
        let purchasable = self.get_purchasable(state);
        let results: Vec<usize> = Vec::new();
        for p in purchasable {
            let mut s = state.clone();
        }
        *results.iter().max().unwrap()
    }

    fn get_purchasable(&self, state: &State) -> Vec<ResourceType> {
        self.blueprint
            .robots
            .iter()
            .filter(|(_, r)| r.can_afford(&state.resources))
            .map(|(_, r)| r.resource_collected)
            .collect()
    }

    pub fn purchase_robot(&mut self, resource_type: ResourceType, state: &mut State) {
        let cost = &self.blueprint.robots[&resource_type].cost;
        for p in cost {
            state.resources.insert(
                p.resource_type,
                state.resources[&p.resource_type] - p.amount,
            );
        }
        state
            .robots
            .insert(resource_type, state.robots[&resource_type] + 1);
    }
}

#[cfg(test)]
mod tests {

    mod processor {
        use crate::{
            blueprint::Blueprint,
            processor::{Processor, State},
            robots::ResourceType,
        };

        macro_rules! blueprint {
            () => {
                {
                    let s = String::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
                    Blueprint::create(&s)
                }
            };
        }

        #[test]
        fn test_get_purchasable() {
            let b = blueprint!();
            let p = Processor::new(b);

            let mut state = State::default();

            state.resources.insert(ResourceType::Ore, 4);

            let result = p.get_purchasable(&state);
            assert_eq!(result.len(), 2);
        }

        #[test]
        fn test_purchase_robot() {
            let b = blueprint!();
            let mut p = Processor::new(b);

            let mut state = State::default();

            state.resources.insert(ResourceType::Ore, 6);

            p.purchase_robot(ResourceType::Clay, &mut state);
            assert_eq!(state.robots[&ResourceType::Clay], 1);
            assert_eq!(state.resources[&ResourceType::Ore], 4);

            p.purchase_robot(ResourceType::Ore, &mut state);
            assert_eq!(state.robots[&ResourceType::Ore], 2);
            assert_eq!(state.resources[&ResourceType::Ore], 0);
        }
    }
}
