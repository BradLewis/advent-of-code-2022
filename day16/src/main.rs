use day16::cave::CaveSolver;
use std::fs;

fn main() {
    let s = fs::read_to_string("test_input.txt").expect("File not found");
    let mut cave_solver = CaveSolver::from_string(s);
    cave_solver.minimise();
    cave_solver.print();
}
