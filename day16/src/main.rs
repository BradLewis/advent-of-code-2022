use day16::cave::{Cave, Name, State};
use std::{collections::HashSet, fs};

fn main() {
    let s = fs::read_to_string("test_input.txt").expect("File not found");
    let mut cave = Cave::from_string(s);
    cave.minimise();
    let mut state = State {
        cave: &cave,
        position: Name(*b"AA"),
        iteration: 0,
        max_iterations: 30,
        total_pressure: 0,
        open_valves: HashSet::new(),
    };
    let result = state.run();
}
