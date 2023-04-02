use std::fs;

use day20::decode;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = decode(&s);

    println!("Part1: {}", result);
}
