use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn from_snafu(s: &str) -> i64 {
    let mut r = 0;
    for c in s.chars() {
        match c {
            '2' => r = r*5 + 2,
            '1' => r = r*5 + 1,
            '0' => r = r*5 + 0,
            '-' => r = r*5 + -1,
            '=' => r = r*5 + -2,
            _ => (),
        }
    }
    r
}

fn to_snafu(mut n: i64) -> String {
    let mut s = String::new();
    while n != 0 {
        let d = (n % 5) as usize;
        s += &"012=-"[d..][..1];
        n += [0,1,2,-2,1][d];
        n /= 5;
    }
    s.chars().rev().collect()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input.lines() {
        sum += from_snafu(line);
    }
    to_snafu(sum)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
