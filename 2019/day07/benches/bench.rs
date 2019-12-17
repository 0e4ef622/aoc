#![feature(test)]

#[path = "../src/solution.rs"]
mod solution;

extern crate test;

const INPUT: &'static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("d7p1_me", |b| b.iter(|| part1(INPUT)));
    c.bench_function("d7p2_me", |b| b.iter(|| solution::part2(INPUT)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
