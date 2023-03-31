use std::fs;

use day19::{part1, part2};

fn main() {
    let s = fs::read_to_string("input_part2.txt").expect("File not found!");

    let result = part2(&s);
    println!("Answer is {}", result);
}
