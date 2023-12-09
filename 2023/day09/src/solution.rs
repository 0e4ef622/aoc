use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn predict(a: &mut [i32]) -> i32 {
    if a.iter().all_equal() {
        return a[0];
    }
    let n = a.len();
    for i in 0..n-1 {
        a[i] = a[i+1] - a[i];
    }
    a[n-1] + predict(&mut a[..n-1])
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    let mut nums = vec![];
    for line in input.lines() {
        nums.extend(line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()));
        ans += predict(&mut nums);
        nums.clear();
    }
    ans
}

fn predict2(a: &mut [i32]) -> i32 {
    if a.iter().all_equal() {
        return a[0];
    }
    let n = a.len();
    let first = a[0];
    for i in 0..n-1 {
        a[i] = a[i+1] - a[i];
    }
    first - predict2(&mut a[..n-1])
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    let mut nums = vec![];
    for line in input.lines() {
        nums.extend(line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()));
        ans += predict2(&mut nums);
        nums.clear();
    }
    ans
}
