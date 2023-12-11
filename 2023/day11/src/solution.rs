use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn solve(input: &str, expansion: usize) -> i64 {
    let g = input.lines().map(|x| x.as_bytes()).cv();
    let mut rows = vec![];
    let mut cols = vec![];
    for i in 0..g.len() {
        let mut has_g = false;
        for j in 0..g[i].len() {
            if g[i][j] == b'#' {
                has_g = true;
                break;
            }
        }
        if !has_g {
            rows.push(i);
        }
    }
    for j in 0..g.len() {
        let mut has_g = false;
        for i in 0..g[j].len() {
            if g[i][j] == b'#' {
                has_g = true;
                break;
            }
        }
        if !has_g {
            cols.push(j);
        }
    }
    let mut coords = vec![];
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == b'#' {
                let ii = i + expansion*rows.iter().filter(|&&x| x < i).count();
                let jj = j + expansion*cols.iter().filter(|&&x| x < j).count();
                coords.push((ii as i64, jj as i64));
            }
        }
    }
    let mut ans = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let c1 = coords[i];
            let c2 = coords[j];
            ans += (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs();
        }
    }
    ans
}
pub fn part1(input: &str) -> impl std::fmt::Display {
    solve(input, 1)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve(input, 999999)
}
