use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn possible(d: &[&str], pat: &str) -> usize {
    let mut dp = vec![0; pat.len() + 1];
    dp[0] = 1;

    for e in 1..=pat.len() {
        for &p in d {
            if e >= p.len() && pat[e - p.len()..e] == *p {
                dp[e] += dp[e-p.len()];
            }
        }
    }

    *dp.last().unwrap()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let ss = input.split("\n\n").collect::<Vec<_>>();
    let designs = ss[0].split(", ").collect::<Vec<_>>();

    let mut ans = 0;
    for pat in ss[1].lines() {
        ans += (possible(&designs, pat) != 0) as usize;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let ss = input.split("\n\n").collect::<Vec<_>>();
    let designs = ss[0].split(", ").collect::<Vec<_>>();

    let mut ans = 0;
    for pat in ss[1].lines() {
        ans += possible(&designs, pat);
    }
    ans
}
