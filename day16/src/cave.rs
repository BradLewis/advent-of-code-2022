use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub [u8; 2]);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

impl Name {
    fn from_string(s: String) -> Self {
        Self(s.as_bytes().try_into().expect("Name was incorrect size"))
    }

    fn from_str(s: &str) -> Self {
        Self::from_string(s.to_string())
    }
}
#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    open: bool,
    flow_rate: i32,
    connections: HashMap<Name, u32>,
}

impl Valve {
    pub fn combine_connections(&mut self, valve: &Valve) {
        if !self.connections.contains_key(&valve.name) {
            return;
        }
        self.connections.remove(&valve.name);
        for (connection, distance) in valve.connections.iter() {
            if connection == &self.name {
                continue;
            }
            if self.connections.contains_key(connection) {
                self.connections.insert(
                    *connection,
                    (distance + 1).min(self.connections[connection]),
                );
            } else {
                self.connections.insert(*connection, distance + 1);
            }
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    valves: HashMap<Name, Valve>,
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
                    open: false,
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

    pub fn minimise(&mut self) {
        self.contract_zero_valves();
        let mut zero_valves: Vec<Valve> = Vec::new();
        let aa = &Name(*b"AA");
        for name in self.zero_valves.iter() {
            if name == aa {
                let v = self.valves.get(name).unwrap().clone();
                zero_valves.push(v);
            } else {
                let v = self.valves.remove(name).unwrap();
                zero_valves.push(v);
            }
        }
        for (name, valve) in self.valves.iter_mut() {
            if name == aa {
                continue;
            }
            for zv in zero_valves.iter() {
                valve.combine_connections(zv);
            }
        }
    }

    fn contract_zero_valves(&mut self) {
        for i in 0..self.zero_valves.len() {
            for j in 0..self.zero_valves.len() {
                if i == j {
                    continue;
                }
                let name1 = &self.zero_valves[i];
                let name2 = &self.zero_valves[j];
                let v2 = self.valves.remove(name2).unwrap();

                let v1 = self.valves.get_mut(name1).unwrap();
                v1.combine_connections(&v2);
                self.valves.insert(v2.name, v2);
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

    fn calculate_distance_matrix(&self) -> Vec<Vec<u32>> {
        let index_map = self.generate_valve_index_map();
        let len = self.valves.len();
        let mut result = vec![vec![u32::MAX; len]; len];
        for (name, valve) in self.valves.iter() {
            for (connection, distance) in valve.connections.iter() {
                let index_y = index_map[name];
                let index_x = index_map[connection];
                result[index_y][index_x] = *distance;
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

    fn generate_valve_index_map(&self) -> HashMap<Name, usize> {
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

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub cave: &'a Cave,
    pub position: Name,
    pub max_iterations: u32,
    pub iteration: u32,
    pub total_pressure: i32,
    pub open_valves: HashSet<usize>,
}

impl State<'_> {
    pub fn run(&mut self) -> i32 {
        let distance_matrix = self.cave.calculate_distance_matrix();
        let valve_map = self.cave.generate_valve_index_map();
        let binding = valve_map.clone();
        let index_map: HashMap<_, _> = binding.iter().map(|(k, v)| (v, k)).collect();

        let mut valve_name = &Name(*b"AA");
        let mut result = 0;
        loop {
            if self.iteration == self.max_iterations {
                break;
            }
            let mut current_best_value = 0;
            let mut current_best_distance = 0;
            let mut current_best_index = 0;
            let current_row = &distance_matrix[valve_map[&valve_name]];

            for (i, distance) in current_row.iter().enumerate() {
                if self.open_valves.contains(&i) {
                    continue;
                }
                if *distance >= self.max_iterations - self.iteration {
                    continue;
                }
                let n = index_map[&i];
                let valve = &self.cave.valves[n];
                let expected_outcome =
                    (self.max_iterations - self.iteration - distance - 1) as i32 * valve.flow_rate;
                if expected_outcome > current_best_value {
                    current_best_value = expected_outcome;
                    current_best_distance = *distance;
                    valve_name = n;
                    current_best_index = i;
                }
            }

            result += current_best_value;
            self.open_valves.insert(current_best_index);
            self.iteration += current_best_distance + 1;
        }
        result
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
            open: false,
            connections: HashMap::from([(Name(*b"EE"), 1), (Name(*b"GG"), 1)]),
        };
        let mut valve_gg = Valve {
            name: Name(*b"GG"),
            flow_rate: 0,
            open: false,
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
    fn test_contract_zero_valves() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(s);
        cave.contract_zero_valves();
        let valve_ff = cave.valves.get(&Name(*b"FF")).unwrap();
        assert_eq!(
            valve_ff.connections,
            HashMap::from([(Name(*b"EE"), 1), (Name(*b"HH"), 2)])
        );

        let valve_gg = cave.valves.get(&Name(*b"GG")).unwrap();
        assert_eq!(
            valve_gg.connections,
            HashMap::from([(Name(*b"HH"), 1), (Name(*b"EE"), 2)])
        );

        let valve_aa = cave.valves.get(&Name(*b"AA")).unwrap();
        assert_eq!(
            valve_aa.connections,
            HashMap::from([(Name(*b"JJ"), 2), (Name(*b"BB"), 1), (Name(*b"DD"), 1)])
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
            HashMap::from([
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
    fn test_run() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(s);
        let mut state = State {
            cave: &cave,
            position: Name(*b"AA"),
            iteration: 0,
            max_iterations: 30,
            total_pressure: 0,
            open_valves: HashSet::new(),
        };
        let result = state.run();
        assert_eq!(result, 1651);
        Ok(())
    }
}
