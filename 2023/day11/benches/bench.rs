const INPUT: &'static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};
use day11::{solution, unsafe_solution};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day11_p1_me", |b| b.iter(|| solution::part1(INPUT)));
    c.bench_function("day11_p2_me", |b| b.iter(|| solution::part2(INPUT)));
    c.bench_function("day11_p1_unsafe_me", |b| b.iter(|| unsafe_solution::part1(INPUT)));
    c.bench_function("day11_p2_unsafe_me", |b| b.iter(|| unsafe_solution::part2(INPUT)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
