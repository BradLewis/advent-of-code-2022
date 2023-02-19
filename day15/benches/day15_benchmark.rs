use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use day15::{part1, part2};

fn part1_benchmark(c: &mut Criterion) {
    let s = fs::read_to_string("input.txt").expect("File not found");
    c.bench_function("part 1", |b| b.iter(|| part1(&s, 2000000)));
}

fn part2_benchmark(c: &mut Criterion) {
    let s = fs::read_to_string("test_input.txt").expect("File not found");
    c.bench_function("part 2", |b| b.iter(|| part2(&s, 4000000)));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
