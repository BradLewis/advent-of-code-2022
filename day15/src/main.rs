use day15::{part1, part2};
use std::fs;

fn main() {
    let _s = fs::read_to_string("input.txt").expect("File not found");
    // let result = part1(&s, 2000000);
    // println!("{}", result);
    firestorm::bench("./flames/", profile_p1).unwrap();
    // firestorm::bench("./flames/", profile_p2).unwrap();

    // let result = part2(&s, 4000000);
    // println!("{}", result);
}

fn profile_p1() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    // let result = part1(&s, 2000000);
    // println!("{}", result);

    let result = part1(&s, 2000000);
    println!("{}", result);
}

fn profile_p2() {
    let s = fs::read_to_string("test_input.txt").expect("File not found");
    // let result = part1(&s, 2000000);
    // println!("{}", result);

    let result = part2(&s, 20);
    println!("{}", result);
}
