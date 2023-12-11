use std::collections::*;
use itertools::{iproduct, Itertools};
use util::*;

pub fn solve(input: &str, expansion: i64) -> i64 {
    let g = input.lines().map(|x| x.as_bytes()).cv();
    let mut x = vec![];
    let mut y = vec![];
    for (i, l) in g.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == b'#' {
                y.push(i as i64);
                x.push(j as i64);
            }
        }
    }
    x.sort_unstable();
    expand(&mut x, expansion);
    expand(&mut y, expansion);
    sum_diff(&x) + sum_diff(&y)
}

fn expand(v: &mut [i64], f: i64) {
    let mut p = 0;
    let mut c = 0;
    for x in &mut *v {
        c += (*x - p - 1).max(0);
        p = *x;
        *x += c*f;
    }
}

fn sum_diff(v: &[i64]) -> i64 {
    let mut sum: i64 = v.iter().sum();
    let mut cnt = v.len() as i64;
    let mut total = 0;
    for &x in v {
        sum -= x;
        cnt -= 1;
        total += sum - cnt*x;
    }
    total
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve(input, 1)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve(input, 999999)
}
