use itertools::{iproduct, Itertools};
use std::collections::*;

pub fn solve(input: &str, expansion: i64) -> i64 {
    let mut x = [0; 512];
    let mut y = [0; 512];
    let mut glen = 0;
    for (i, l) in input.lines().enumerate() {
        let mut j = 0;
        for s in l.split('#') {
            j += s.len() + 1;
            unsafe {
                *y.get_unchecked_mut(glen) = i as i64 - 1;
                *x.get_unchecked_mut(glen) = j as i64 - 1;
            }
            glen += 1;
        }
        glen -= 1;
    }
    let x = unsafe { x.get_unchecked_mut(..glen) };
    let y = unsafe { y.get_unchecked_mut(..glen) };
    radsort::sort(x);
    expand(x, expansion);
    expand(y, expansion);
    sum_diff(&x) + sum_diff(&y)
}

fn expand(v: &mut [i64], f: i64) {
    let mut p = 0;
    let mut c = 0;
    for x in v {
        c += (*x - p - 1).max(0);
        p = *x;
        *x += c * f;
    }
}

fn sum_diff(v: &[i64]) -> i64 {
    let mut sum: i64 = v.iter().sum();
    let mut cnt = v.len() as i64;
    let mut total = 0;
    for &x in v {
        sum -= x;
        cnt -= 1;
        total += sum - cnt * x;
    }
    total
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve(input, 1)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve(input, 999999)
}

pub fn run(input: &str) -> impl std::fmt::Display { part1(input) }
