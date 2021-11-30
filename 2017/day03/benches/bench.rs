#![feature(test)]
#![allow(warnings)]
include!("../src/solution.rs");

extern crate test;

const INPUT: &'static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1_me", |b| b.iter(|| part1(INPUT)));
    c.bench_function("p2_me", |b| b.iter(|| part2(INPUT)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
