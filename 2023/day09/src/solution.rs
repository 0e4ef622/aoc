use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn predict(a: &[i64]) -> i64 {
    if a.iter().all_equal() {
        return a[0];
    }
    let diff = a.windows(2).map(|a| a[1]-a[0]).cv();
    a.last().unwrap() + predict(&diff)
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let nums = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).cv();
        ans += predict(&nums);
    }
    ans
}

fn predict2(a: &[i64]) -> i64 {
    if a.iter().all_equal() {
        return a[0];
    }
    let diff = a.windows(2).map(|a| a[1]-a[0]).cv();
    a.first().unwrap() - predict2(&diff)
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let nums = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).cv();
        ans += predict2(&nums);
    }
    ans
}
