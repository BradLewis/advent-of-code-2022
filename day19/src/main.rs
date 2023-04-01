use std::fs;

use day19::{part1, part2};

fn main() {
    let s1 = fs::read_to_string("input.txt").expect("File not found!");

    let result = part1(&s1);
    println!("Part1 answer is {}", result);

    let s2 = fs::read_to_string("input_part2.txt").expect("File not found!");
    let result = part2(&s2);
    println!("Part2 answer is {}", result);
}
