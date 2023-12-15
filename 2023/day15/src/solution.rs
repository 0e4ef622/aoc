use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn hash(s: &str) -> i64 {
    let mut v = 0;
    for b in s.as_bytes() {
        v += *b as i64;
        v = v*17 % 256;
    }
    v
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut r = 0;
    for x in input.trim().split(',') {
        r += hash(x);
    }
    r
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut r = 0;
    let mut boxes: Vec<Vec<(&str, i64)>> = vec![vec![]; 256];
    for x in input.trim().split(',') {
        if x.contains('-') {
            let label = &x[..x.len()-1];
            let b = hash(label);
            if let Some((i, _)) = boxes[b as usize].iter().enumerate().find(|(_, x)| x.0 == label) {
                boxes[b as usize].remove(i);
            }
        } else {
            let label = &x[..x.len()-2];
            let f = x[x.len()-1..].parse::<i64>().unwrap();
            let b = hash(label);
            if let Some((i, _)) = boxes[b as usize].iter().enumerate().find(|(_, x)| x.0 == label) {
                boxes[b as usize][i] = (label, f);
            } else {
                boxes[b as usize].push((label, f));
            }
        }
    }
    let mut ans = 0;
    for (j, b) in boxes.iter().enumerate() {
        for (i, x) in b.iter().enumerate() {
            ans += (j as i64 + 1)*(i as i64+1)*x.1;
        }
    }
    ans
}
