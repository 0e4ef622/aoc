#!/bin/bash

set -e

day=$1;
if [ ${#day} = 1 ]; then
    day=0$day;
fi

if [ -e day$day ]; then
    echo Directory exists >&2
    exit 1
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
itertools = "*"
util = { path = "../../util" }' > Cargo.toml;

mkdir benches src;
echo 'const INPUT: &'\''static str = include_str!("../in");

use criterion::{criterion_group, criterion_main, Criterion};
use day'$day'::solution::{part1, part2};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day'$day'_p1_me", |b| b.iter(|| black_box(part1(black_box(INPUT)))));
    c.bench_function("day'$day'_p2_me", |b| b.iter(|| black_box(part2(black_box(INPUT)))));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);' > benches/bench.rs;

echo '#![allow(warnings)]
use std::io::Read;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    if args.len() <= 1 {
        let p1 = day'$day'::solution::part1(&input);
        println!("part 1: {}", p1);
        let p2 = day'$day'::solution::part2(&input);
        println!("part 2: {}", p2);
    } else if args[1] == "2" {
        let p2 = day'$day'::solution::part2(&input);
        println!("{}", p2);
    } else {
        let p1 = day'$day'::solution::part1(&input);
        println!("{}", p1);
    }
}' > src/main.rs

echo '#![allow(warnings)]
pub mod solution;
' > src/lib.rs

echo 'use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}' > src/solution.rs;
