const INPUT: &'static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};
use day24::solution::{part1, part2};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day24_p1_me", |b| b.iter(|| black_box(part1(black_box(INPUT)))));
    c.bench_function("day24_p2_me", |b| b.iter(|| black_box(part2(black_box(INPUT)))));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
