use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use bimap::BiMap;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub [u8; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

impl Name {
    fn from_string(s: String) -> Self {
        Self(s.as_bytes().try_into().expect("Name was incorrect size"))
    }
}
#[derive(Debug, Clone)]
pub struct Valve {
    name: Name,
    flow_rate: i32,
    connections: HashMap<Name, u32>,
}

impl Valve {
    pub fn combine_connections(&mut self, valve: &Valve) {
        if !self.connections.contains_key(&valve.name) {
            return;
        }
        let d = self.connections.remove(&valve.name).unwrap();
        for (connection, distance) in valve.connections.iter() {
            if connection == &self.name {
                continue;
            }
            if self.connections.contains_key(connection) {
                self.connections.insert(
                    *connection,
                    (distance + d).min(self.connections[connection]),
                );
            } else {
                self.connections.insert(*connection, distance + d);
            }
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    pub valves: HashMap<Name, Valve>,
    zero_valves: Vec<Name>,
}

impl Cave {
    pub fn from_string(s: String) -> Self {
        let mut zero_valves: Vec<Name> = Vec::new();
        let valves: HashMap<_, _> = s
            .lines()
            .map(|l| {
                let flow = parse::<i32>(l, r"\d+");
                let mut valves_in_line: Vec<_> = parse::<String>(l, r"[A-Z]{2}")
                    .into_iter()
                    .map(Name::from_string)
                    .collect();
                let name = valves_in_line.remove(0);
                let flow_rate = flow[0];
                if flow_rate == 0 {
                    zero_valves.push(name);
                }
                let valve = Valve {
                    name,
                    connections: valves_in_line.into_iter().map(|v| (v, 1)).collect(),
                    flow_rate,
                };
                (name, valve)
            })
            .collect();
        Self {
            valves,
            zero_valves,
        }
    }

    fn remove_valve(&mut self, valve: &Valve) {
        for (_, v) in self.valves.iter_mut() {
            v.combine_connections(valve);
        }
    }

    pub fn minimise(&mut self) {
        let aa = Name(*b"AA");
        for i in 0..self.zero_valves.len() {
            let zv = self.valves.remove(&self.zero_valves[i]).unwrap();
            self.remove_valve(&zv);
            if self.zero_valves[i] == aa {
                self.valves.insert(aa, zv);
            }
        }
    }

    pub fn print(&self) {
        for (_, v) in self.valves.iter() {
            print!("{} =>", v.name);
            for (k, d) in v.connections.iter() {
                print!("[{},{}]", k, d);
            }
            println!();
        }
    }

    pub fn calculate_distance_matrix(&self) -> Vec<Vec<u32>> {
        let index_map = self.generate_valve_index_map();
        let len = self.valves.len();
        let mut result = vec![vec![u32::MAX; len]; len];
        for (name, valve) in self.valves.iter() {
            for (connection, distance) in valve.connections.iter() {
                let index_y = index_map.get_by_left(name).unwrap();
                let index_x = index_map.get_by_left(connection).unwrap();
                result[*index_y][*index_x] = *distance;
            }
        }
        (0..len).for_each(|i| {
            result[i][i] = 0;
        });

        for i in 0..len {
            for j in 0..len {
                for k in 0..len {
                    if result[i][k] == u32::MAX {
                        continue;
                    }
                    if result[k][j] == u32::MAX {
                        continue;
                    }
                    if result[i][j] > result[i][k] + result[k][j] {
                        result[i][j] = result[i][k] + result[k][j]
                    }
                }
            }
        }
        result
    }

    pub fn generate_valve_index_map(&self) -> BiMap<Name, usize> {
        let mut names: Vec<_> = self.valves.iter().map(|v| v.0).collect();
        names.sort();
        names
            .iter()
            .enumerate()
            .map(|(i, name)| (**name, i))
            .collect()
    }
}

fn parse<T: FromStr>(s: &str, re: &str) -> Vec<T> {
    let regex: Regex = Regex::new(re).unwrap();
    regex
        .find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    target: Name,
    cost: u32,
    reward: i32,
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub cave: &'a Cave,
    pub distance_matrix: &'a Vec<Vec<u32>>,
    pub valve_index_map: &'a BiMap<Name, usize>,
    pub position: Name,
    pub max_iterations: u32,
    pub iteration: u32,
    pub total_pressure: i32,
    pub open_valves: HashSet<Name>,
    pub cached_values: Vec<Vec<i32>>,
}

impl State<'_> {
    fn iterations_left(&self) -> u32 {
        self.max_iterations - self.iteration
    }

    fn set_cache(&mut self, iteration: u32, cost: u32, position: Name, value: i32) {
        for i in 0..cost {
            let c = self.cached_values[(iteration + i) as usize]
                [*self.valve_index_map.get_by_left(&position).unwrap()];
            if value > c {
                self.cached_values[(self.iteration + i) as usize + 1]
                    [*self.valve_index_map.get_by_left(&self.position).unwrap()] = value;
            }
        }
    }

    fn merge_cache(&mut self, cache: &Vec<Vec<i32>>) {
        for i in 0..cache.len() {
            for j in 0..cache[0].len() {
                if self.cached_values[i][j] < cache[i][j] {
                    self.cached_values[i][j] = cache[i][j];
                }
            }
        }
    }

    pub fn calculate_best_moves(&mut self) -> (Self, Vec<Move>) {
        let mut best_moves = Vec::new();
        let mut best_state = self.clone();
        let mut best_pressure = 0;

        let moves = self.calculate_moves();
        for m in moves.iter() {
            let mut next = self.apply_move(m);
            self.set_cache(self.iteration, m.cost, self.position, next.total_pressure);
            if next.iterations_left() == 0 {
                continue;
            }
            let (next, mut next_moves) = next.calculate_best_moves();
            next_moves.push(*m);
            self.merge_cache(&next.cached_values);
            if next.total_pressure > best_pressure {
                best_pressure = next.total_pressure;
                best_moves = next_moves;
                best_state = next;
            }
        }

        (best_state, best_moves)
    }

    fn apply_move(&self, m: &Move) -> Self {
        let mut next = self.clone();
        next.total_pressure += m.reward;
        next.position = m.target;
        next.iteration += m.cost;
        next.open_valves.insert(m.target);

        next
    }

    fn calculate_moves(&self) -> Vec<Move> {
        self.distance_matrix[*self.valve_index_map.get_by_left(&self.position).unwrap()]
            .iter()
            .enumerate()
            .filter_map(|(i, distance)| {
                if *distance == 0 {
                    return None;
                }
                if *distance == u32::MAX {
                    return None;
                }
                let cost = *distance + 1;
                if cost > self.iterations_left() {
                    return None;
                }
                let target = *self.valve_index_map.get_by_right(&i).unwrap();
                if self.open_valves.contains(&target) {
                    return None;
                }
                let valve = &self.cave.valves[&target];
                if valve.flow_rate == 0 {
                    return None;
                }
                let reward = valve.flow_rate * (self.iterations_left() as i32 - cost as i32);
                if reward == 0 {
                    return None;
                }
                Some(Move {
                    target,
                    reward,
                    cost,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_parse_line() -> Result<(), String> {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let flow = parse::<i32>(input, r"\d+");
        let valves = parse::<String>(input, r"[A-Z]{2}");
        assert_eq!(flow, vec![0]);
        assert_eq!(
            valves,
            vec![
                "AA".to_string(),
                "DD".to_string(),
                "II".to_string(),
                "BB".to_string()
            ]
        );
        Ok(())
    }

    #[test]
    fn test_load_cave() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(s);
        assert_eq!(cave.valves.len(), 10);
        assert_eq!(cave.valves[&Name(*b"AA")].connections.len(), 3);
        Ok(())
    }

    #[test]
    fn test_minimising_cave() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.minimise();
        assert_eq!(cave.valves.len(), 7);
        let ee_valve = &cave.valves[&Name(*b"EE")];
        assert_eq!(ee_valve.connections.len(), 2);
        assert_eq!(ee_valve.connections[&Name(*b"HH")], 3);
        Ok(())
    }

    #[test]
    fn test_combine_zero_connections() -> Result<(), String> {
        let mut valve_ff = Valve {
            name: Name(*b"FF"),
            flow_rate: 0,
            connections: HashMap::from([(Name(*b"EE"), 1), (Name(*b"GG"), 1)]),
        };
        let mut valve_gg = Valve {
            name: Name(*b"GG"),
            flow_rate: 0,
            connections: HashMap::from([(Name(*b"HH"), 1), (Name(*b"FF"), 1)]),
        };
        valve_ff.combine_connections(&valve_gg);
        assert_eq!(
            valve_ff.connections,
            HashMap::from([(Name(*b"EE"), 1), (Name(*b"HH"), 2)])
        );
        valve_gg.combine_connections(&valve_ff);
        assert_eq!(
            valve_gg.connections,
            HashMap::from([(Name(*b"HH"), 1), (Name(*b"EE"), 2)])
        );
        Ok(())
    }

    #[test]
    fn test_get_distance_matrix() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.minimise();
        let dm = cave.calculate_distance_matrix();
        assert_eq!(
            dm,
            vec![
                vec![0, 1, 2, 1, 2, 5, 2],
                vec![u32::MAX, 0, 1, 2, 3, 6, 3],
                vec![u32::MAX, 1, 0, 1, 2, 5, 4],
                vec![u32::MAX, 2, 1, 0, 1, 4, 3],
                vec![u32::MAX, 3, 2, 1, 0, 3, 4],
                vec![u32::MAX, 6, 5, 4, 3, 0, 7],
                vec![u32::MAX, 3, 4, 3, 4, 7, 0]
            ]
        );
        Ok(())
    }

    #[test]
    fn test_generate_valve_index_map() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let mut cave = Cave::from_string(s);
        cave.minimise();
        let vim = cave.generate_valve_index_map();
        assert_eq!(
            vim,
            BiMap::from_iter([
                (Name(*b"AA"), 0),
                (Name(*b"BB"), 1),
                (Name(*b"CC"), 2),
                (Name(*b"DD"), 3),
                (Name(*b"EE"), 4),
                (Name(*b"HH"), 5),
                (Name(*b"JJ"), 6),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_calculate_best_moves() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.minimise();
        let mut state = State {
            cave: &cave,
            distance_matrix: &cave.calculate_distance_matrix(),
            valve_index_map: &cave.generate_valve_index_map(),
            position: Name(*b"AA"),
            iteration: 0,
            max_iterations: 30,
            total_pressure: 0,
            open_valves: HashSet::new(),
            cached_values: Vec::new(),
        };
        let (state, _) = state.calculate_best_moves();
        assert_eq!(state.total_pressure, 1651);
        Ok(())
    }

    #[test]
    fn test_calculate_moves() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.minimise();
        let state = State {
            cave: &cave,
            distance_matrix: &cave.calculate_distance_matrix(),
            valve_index_map: &cave.generate_valve_index_map(),
            position: Name(*b"AA"),
            iteration: 0,
            max_iterations: 3,
            total_pressure: 0,
            open_valves: HashSet::new(),
            cached_values: Vec::new(),
        };
        let result = state.calculate_moves();
        assert_eq!(
            result,
            vec![
                Move {
                    target: Name(*b"BB"),
                    cost: 2,
                    reward: 13
                },
                Move {
                    target: Name(*b"DD"),
                    cost: 2,
                    reward: 20
                }
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.minimise();
        let mut state = State {
            cave: &cave,
            distance_matrix: &cave.calculate_distance_matrix(),
            valve_index_map: &cave.generate_valve_index_map(),
            position: Name(*b"AA"),
            iteration: 0,
            max_iterations: 15,
            total_pressure: 0,
            open_valves: HashSet::new(),
            cached_values: Vec::new(),
        };
        let (state, _) = state.calculate_best_moves();
        assert_eq!(state.total_pressure, 1707);
        Ok(())
    }
}
