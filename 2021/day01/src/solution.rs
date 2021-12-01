use std::collections::*;
use rand::random;
use serde_scan::scan as s;

use util::*;

pub fn _part1(input: &str) -> impl std::fmt::Display {
    let v: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut c = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            c += 1;
        }
    }
    c
}

pub fn _part2(input: &str) -> impl std::fmt::Display {
    let v: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let v = v.windows(3).map(|x| x.iter().sum::<i64>()).collect::<Vec<_>>();
    let mut c = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            c += 1;
        }
    }
    c
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .app(|i| i.clone().zip(i.skip(1)))
        .filter(|(x, y)| x < y)
        .count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collectv()
        .appr(|v| v.windows(3))
        .map(|w| w.iter().sum::<i64>())
        .app(|i| i.clone().zip(i.skip(1)))
        .filter(|(x, y)| x < y)
        .count()
}
