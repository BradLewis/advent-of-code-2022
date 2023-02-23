use day16::cave::{run_with_elephant, Cave, Name, State};
use std::{collections::HashSet, fs};

fn part1(s: &str) {
    let mut cave = Cave::from_str(s);
    cave.minimise();
    cave.print();
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
    let (state, mut moves) = state.calculate_best_moves(u32::MAX);
    moves.reverse();
    println!("{:?} - {}", moves, state.total_pressure);
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    part1(&s);
    part2(&s);
}

fn part2(s: &str) {
    let result = run_with_elephant(s);
    println!("{}", result);
}
