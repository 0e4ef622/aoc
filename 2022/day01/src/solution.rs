use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut a = 0;
    let mut mx = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            a = 0;
            continue;
        }
        let x = line.trim().parse::<i64>().unwrap();
        a += x;
        mx = a.max(mx);
    }
    mx
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut a = 0;
    let mut asdf = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            asdf.push(a);
            a=0;
            continue;
        }
        let x = line.trim().parse::<i64>().unwrap();
        a += x;
    }
    asdf.sort();
    asdf.reverse();
    asdf[0] + asdf[1] + asdf[2]
}
