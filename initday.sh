#!/bin/bash

set -e

day=$1;
if [ ${#day} = 1 ]; then
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
serde_scan = "*"
util = { path = "../../util" }' > Cargo.toml;

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
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}' > src/solution.rs;
