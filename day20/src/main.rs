use std::fs;

use day20::decode;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = decode(&s, 1, 1);
    println!("Part1: {}", result);

    let result = decode(&s, 811_589_153, 10);
    println!("Part2: {}", result);
}
