#!/bin/bash

day=$1;
if ${#day} == 1; then
    day=0$day;
fi

mkdir day$day;
cd day$day;
echo '[package]
name = "'day$day'"
version = "0.1.0"
authors = ["Matthew Tran <0e4ef622@gmail.com>"]
edition = "2021"

[dev-dependencies]
criterion = "*"

[[bench]]
name = "bench"
harness = false

[dependencies]
rand = "*"
serde_scan = "*"' > Cargo.toml;

mkdir benches src;
echo '#![feature(test)]
include!("../src/solution.rs");

extern crate test;

const INPUT: &'\''static str = include_str!("../in");

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
criterion_main!(benches);' > benches/bench.rs;

echo '#![allow(warnings)]
use std::io::Read;
mod solution;
// const INPUT: &'\''static str = include_str!("../in");
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    let p1 = solution::part1(&input);
    println!("part 1: {}", p1);
    let p2 = solution::part2(&input);
    println!("part 2: {}", p2);
}' > src/main.rs

echo 'use std::collections::*;
use rand::random;
use serde_scan::scan as s;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

pub fn part1(input: &str) -> impl std::fmt::Display + '\''_ {
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display + '\''_ {
    0
}' > src/solution.rs;
