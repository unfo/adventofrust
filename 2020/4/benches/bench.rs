use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc202004::*;


const INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_input", |b| b.iter(|| parse_input(black_box(INPUT))));
    let input = parse_input(INPUT).unwrap();
    c.bench_function("part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("part2", |b| b.iter(|| part2(black_box(&input))));
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);