use std::fs;

fn main() {
    let content = fs::read_to_string("inputs.txt").expect("Cannot load file");

    let counts = content
        .split("\n\n")
        .map(|chunk| -> usize { chunk.split("\n").map(|row| row.parse().unwrap_or(0)).sum() });

    let mut v = counts.collect::<Vec<_>>();

    v.sort();
    let len = v.len();

    println!("{}", v[len - 1] + v[len - 2] + v[len - 3]);
}

