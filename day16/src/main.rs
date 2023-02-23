use day16::cave::{Cave, Name, State};
use std::{collections::HashSet, fs};

fn main() {
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
    };
    let (state, moves) = state.calculate_best_moves();
    println!("{:?} - {}", moves, state.total_pressure);
}
