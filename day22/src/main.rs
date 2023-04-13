use day22::part1;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s);
    println!("{}", result);
}
