use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn is_rre(g: &[String], i: usize) -> usize {
    let mut c = 0;
    for j in 0..i {
        let k = i as i64-j as i64-1;
        let l = i as i64+j as i64;
        if k >= 0 && l < g.len() as i64 {
            c += (0..g[0].len()).filter(|&m| g[k as usize][m..][..1] != g[l as usize][m..][..1]).count();
        }
    }
    c
}
fn is_cre(g: &[String], i: usize) -> usize {
    let mut c = 0;
    for j in 0..i {
        let k = i as i64-j as i64-1;
        let l = i as i64+j as i64;
        if k >= 0 && l < g[0].len() as i64 {
            c += g.iter().filter(|r| r[k as usize..][..1] != r[l as usize..][..1]).count();
        }
    }
    c
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for sec in input.split("\n\n") {
        let g = sec.lines().map(String::from).cv();
        for i in 1..g.len() {
            if is_rre(&g, i) == 0 {
                ans += i*100;
            }
        }
        for i in 1..g[0].len() {
            if is_cre(&g, i) == 0 {
                ans += i;
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for sec in input.split("\n\n") {
        let g = sec.lines().map(String::from).cv();
        for i in 1..g.len() {
            if is_rre(&g, i) == 1 {
                ans += i*100;
            }
        }
        for i in 1..g[0].len() {
            if is_cre(&g, i) == 1 {
                ans += i;
            }
        }
    }
    ans
}
