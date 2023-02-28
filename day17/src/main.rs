use std::fs;

use day17::check_height_after;

fn main() {
    let pieces_str = fs::read_to_string("pieces.txt").expect("File not found");
    let chamber_str = fs::read_to_string("input.txt").expect("File not found");
    let result = check_height_after(&chamber_str, &pieces_str, 2022);
    println!("{}", result);

    let result = check_height_after(&chamber_str, &pieces_str, 1000000000000);
    println!("{}", result);
}
