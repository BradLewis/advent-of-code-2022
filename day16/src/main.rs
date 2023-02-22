use day16::cave::Cave;
use std::fs;

fn main() {
    let s = fs::read_to_string("test_input.txt").expect("File not found");
    let mut cave = Cave::from_string(&s);
    cave.minimise();
    cave.print();
}
