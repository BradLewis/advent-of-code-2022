use day18::Graph;
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");

    let graph = Graph::from_string(&s);
    let result = graph.surface_area(true);

    println!("{:?}", result);

    let result = graph.surface_area(false);

    println!("{:?}", result);
}
