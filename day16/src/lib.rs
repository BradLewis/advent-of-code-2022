use regex::Regex;
use std::{collections::HashMap, hash::Hash, str::FromStr};

#[derive(Debug)]
struct Valve {
    name: String,
    open: bool,
    flow_rate: i32,
    connections: Vec<String>,
}

struct Part1 {
    valves: HashMap<String, Valve>,
}

impl Part1 {
    fn from_string(s: &str) -> Self {
        let valves: HashMap<_, _> = s
            .lines()
            .map(|l| {
                let flow = parse::<i32>(l, r"\d+");
                let mut v = parse::<String>(l, r"[A-Z]{2}");
                let name = v.remove(0);
                let valve = Valve {
                    name: name.clone(),
                    connections: v,
                    open: false,
                    flow_rate: flow[0],
                };
                (name, valve)
            })
            .collect();
        Self { valves }
    }

    fn get_next(&self, current_valve: String) -> String {
        let mut current_max_flow = -1;
        let mut current_best = current_valve.clone();
        for v in self.valves[&current_valve].connections.iter() {
            let valve = &self.valves[v];
            if valve.open {
                continue;
            }
            if valve.flow_rate <= current_max_flow {
                continue;
            }
            current_max_flow = valve.flow_rate;
            current_best = valve.name.clone();
        }
        current_best
    }

    fn run(&mut self) -> i32 {
        let mut remaining_minutes = 30;
        let mut total_score = 0;
        let mut current_flow = 0;
        let mut current_valve = "AA".to_string();
        while remaining_minutes != 0 {
            println!("{}: {}", remaining_minutes, current_valve);
            let valve = &mut self.valves.get_mut(&current_valve).unwrap();
            if !valve.open && valve.flow_rate > 0 {
                total_score += current_flow;
                valve.open = true;
                current_flow += valve.flow_rate;
                remaining_minutes -= 1;
            }
            current_valve = self.get_next(current_valve);
            total_score += current_flow;
            remaining_minutes -= 1;
        }
        total_score
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

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_valve_get_next() -> Result<(), String> {
        let mut valves: HashMap<String, Valve> = HashMap::new();
        valves.insert(
            "AA".to_string(),
            Valve {
                name: "AA".to_string(),
                open: false,
                flow_rate: 0,
                connections: vec!["DD".to_string(), "II".to_string()],
            },
        );
        valves.insert(
            "DD".to_string(),
            Valve {
                name: "DD".to_string(),
                open: false,
                flow_rate: 20,
                connections: vec!["AA".to_string()],
            },
        );
        valves.insert(
            "II".to_string(),
            Valve {
                name: "II".to_string(),
                open: false,
                flow_rate: 10,
                connections: vec!["AA".to_string()],
            },
        );
        let p1 = Part1 { valves };
        let result = p1.get_next("AA".to_string());

        assert_eq!(result, "DD");
        Ok(())
    }

    #[test]
    fn test_part1_solve() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut p1 = Part1::from_string(&s);
        let result = p1.run();
        assert_eq!(result, 1651);
        Ok(())
    }

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
}
