use crate::{blueprint::Blueprint, robots::ResourceType};
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
    robots: HashMap<ResourceType, usize>,
    pub resources: HashMap<ResourceType, usize>,
    iteration: usize,
    skips: Vec<ResourceType>,
}

impl State {
    pub fn new() -> Self {
        let mut robots = init_resource_count!();
        robots.insert(ResourceType::Ore, 1);
        Self {
            robots,
            resources: init_resource_count!(),
            iteration: 0,
            skips: Vec::new(),
        }
    }

    fn gather_resources(&mut self) {
        for (&k, &v) in self.robots.iter() {
            self.resources.insert(k, self.resources[&k] + v);
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
    pub max_iterations: usize,
}

impl Processor {
    pub fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            max_iterations: 24,
        }
    }

    pub fn process_turn(&mut self, state: &mut State) -> State {
        state.iteration += 1;
        let purchasable = self.get_purchasable(state);
        state.gather_resources();
        if state.iteration == self.max_iterations {
            return state.clone();
        }
        let mut results: Vec<State> = Vec::new();
        for &p in purchasable.iter() {
            if state.skips.contains(&p) {
                continue;
            }
            let mut s = state.clone();
            s.skips.clear();
            self.purchase_robot(p, &mut s);
            results.push(self.process_turn(&mut s));
        }
        if purchasable.len() < 4 {
            let mut s = state.clone();
            s.skips = purchasable;
            results.push(self.process_turn(&mut s));
        }
        results
            .into_iter()
            .max_by(|x, y| {
                x.resources[&ResourceType::Geode].cmp(&y.resources[&ResourceType::Geode])
            })
            .unwrap()
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

    mod state {
        use crate::{processor::State, robots::ResourceType};

        #[test]
        fn test_gather_resources() {
            let mut s = State::new();
            assert_eq!(s.resources[&ResourceType::Ore], 0);
            s.gather_resources();
            assert_eq!(s.resources[&ResourceType::Ore], 1);
        }
    }

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

        #[test]
        fn test_geode_count() {
            let b = blueprint!();
            let mut p = Processor::new(b);
            let mut state = State::default();

            let result = p.process_turn(&mut state);
            assert_eq!(result.resources[&ResourceType::Geode], 9);
        }
    }
}
