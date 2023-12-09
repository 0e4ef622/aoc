const INPUT: &'static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};
use day09::solution::{part1, part2, part1_funny};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day09_p1_me", |b| b.iter(|| part1(INPUT)));
    c.bench_function("day09_p2_me", |b| b.iter(|| part2(INPUT)));
    c.bench_function("day09_p1_me_funny", |b| b.iter(|| part1_funny(INPUT)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
