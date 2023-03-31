use std::fs;

use day19::part1;

fn main() {
    let s = fs::read_to_string("test_input.txt").expect("File not found!");

    let result = part1(&s);
    println!("Answer is {}", result);
}
