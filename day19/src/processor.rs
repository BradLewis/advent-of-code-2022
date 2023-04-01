use crate::{blueprint::Blueprint, robots::ResourceType};
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    robots: Vec<usize>,
    pub resources: Vec<usize>,
    iteration: usize,
    skips: Vec<ResourceType>,
    wanted_robots: Vec<bool>,
}
impl State {
    pub fn new() -> Self {
        let mut robots = vec![0; ResourceType::COUNT];
        robots[ResourceType::Ore as usize] = 1;
        let wanted_robots = vec![true; ResourceType::COUNT];
        Self {
            robots,
            resources: vec![0; ResourceType::COUNT],
            iteration: 0,
            skips: Vec::new(),
            wanted_robots,
        }
    }
    fn gather_resources(&mut self) {
        for (k, v) in self.robots.iter().enumerate() {
            self.resources[k] += v;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources[ResourceType::Geode as usize]
            .cmp(&other.resources[ResourceType::Geode as usize])
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Processor {
    pub blueprint: Blueprint,
    pub max_iterations: usize,
}

impl Processor {
    pub fn new(blueprint: Blueprint, max_iterations: usize) -> Self {
        Self {
            blueprint,
            max_iterations,
        }
    }

    pub fn process_turn(&mut self, state: &mut State) -> State {
        state.iteration += 1;
        let purchasable = self.get_purchasable(state);
        state.gather_resources();
        if state.iteration == self.max_iterations {
            return state.clone();
        }
        let mut results = BinaryHeap::new();
        for &p in purchasable.iter() {
            if state.skips.contains(&p) {
                continue;
            }
            let mut s = state.clone();
            s.skips.clear();
            self.purchase_robot(p, &mut s);
            results.push(self.process_turn(&mut s));
        }
        if purchasable.len() < self.blueprint.robots.len() {
            let mut s = state.clone();
            s.skips = purchasable;
            results.push(self.process_turn(&mut s));
        }
        results.pop().unwrap()
    }

    fn get_purchasable(&self, state: &State) -> Vec<ResourceType> {
        self.blueprint
            .robots
            .iter()
            .filter(|(rt, _)| state.wanted_robots[**rt as usize])
            .filter(|(_, r)| r.can_afford(&state.resources))
            .map(|(_, r)| r.resource_collected)
            .collect()
    }
    pub fn purchase_robot(&mut self, resource_type: ResourceType, state: &mut State) {
        let cost = &self.blueprint.robots[&resource_type].cost;
        for p in cost {
            state.resources[p.resource_type as usize] -= p.amount;
        }
        let count_robots_of_type = state.robots[resource_type as usize] + 1;
        state.robots[resource_type as usize] = count_robots_of_type;
        if &count_robots_of_type
            == self
                .blueprint
                .max_costs
                .get(&resource_type)
                .unwrap_or(&usize::MAX)
        {
            state.wanted_robots[resource_type as usize] = false;
        }
    }
}

#[cfg(test)]
mod tests {

    mod state {
        use crate::{processor::State, robots::ResourceType};

        #[test]
        fn test_gather_resources() {
            let mut s = State::new();
            assert_eq!(s.resources[ResourceType::Ore as usize], 0);
            s.gather_resources();
            assert_eq!(s.resources[ResourceType::Ore as usize], 1);
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
            let p = Processor::new(b, 24);

            let mut state = State::default();

            state.resources[ResourceType::Ore as usize] = 4;

            let result = p.get_purchasable(&state);
            assert_eq!(result.len(), 2);
        }

        #[test]
        fn test_purchase_robot() {
            let b = blueprint!();
            let mut p = Processor::new(b, 24);

            let mut state = State::default();

            state.resources[ResourceType::Ore as usize] = 6;

            p.purchase_robot(ResourceType::Clay, &mut state);
            assert_eq!(state.robots[ResourceType::Clay as usize], 1);
            assert_eq!(state.resources[ResourceType::Ore as usize], 4);

            p.purchase_robot(ResourceType::Ore, &mut state);
            assert_eq!(state.robots[ResourceType::Ore as usize], 2);
            assert_eq!(state.resources[ResourceType::Ore as usize], 0);
        }

        #[test]
        fn test_geode_count() {
            let b = blueprint!();
            let mut p = Processor::new(b, 24);
            let mut state = State::default();

            let result = p.process_turn(&mut state);
            assert_eq!(result.resources[ResourceType::Geode as usize], 9);
        }
    }
}
