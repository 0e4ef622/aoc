use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn solve(input: &str, expansion: usize) -> i64 {
    let g = input.lines().map(|x| x.as_bytes()).cv();
    let mut rows = Vec::with_capacity(g.len());
    let mut cols = Vec::with_capacity(g[0].len());
    for (i, l) in g.iter().enumerate() {
        let mut has_g = l.contains(&b'#');
        if !has_g {
            rows.push(i);
        }
    }
    for j in 0..g[0].len() {
        let mut has_g = false;
        for i in 0..g.len() {
            if g[i][j] == b'#' {
                has_g = true;
                break;
            }
        }
        if !has_g {
            cols.push(j);
        }
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == b'#' {
                let ii = i + expansion*rows.iter().filter(|&&x| x < i).count();
                let jj = j + expansion*cols.iter().filter(|&&x| x < j).count();
                y.push(ii as i64);
                x.push(jj as i64);
            }
        }
    }
    x.sort_unstable();
    sum_diff(&x) + sum_diff(&y)
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
