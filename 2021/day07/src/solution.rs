use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut n = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).cv();
    n.sort();
    n.iter().map(|v| (v-n[n.len()/2]).abs()).sum::<i64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut n = input.trim().split(',').map(|x| x.parse::<i64>().unwrap()).cv();
    n.sort();
    let s = n.iter().sum::<i64>();
    let m = (s as f64 / (n.len() as f64)).round() as i64;
    n.iter().map(|v| {
        let d = (v-m).abs();
        d*(d+1)/2
    }).sum::<i64>().min(
    n.iter().map(|v| {
        let d = (v-(m+1)).abs();
        d*(d+1)/2
    }).sum::<i64>()).min(
    n.iter().map(|v| {
        let d = (v-(m-1)).abs();
        d*(d+1)/2
    }).sum::<i64>())
}
