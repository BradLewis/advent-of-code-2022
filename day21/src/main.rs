use std::fs;

use day21::{part1, part2};

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");

    let result = part1(&s);
    println!("Part 1: {}", result);

    let result = part2(&s);
    println!("Part 2: {}", result);
}
