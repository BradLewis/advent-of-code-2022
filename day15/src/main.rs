use day15::part1;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s, 2000000);

    println!("{}", result);
}
