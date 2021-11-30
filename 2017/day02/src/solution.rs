use std::collections::*;
use rand::random;
use serde_scan::scan as s;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut ck = 0;
    for line in inlines {
        let v: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        ck += v.iter().max().unwrap() - v.iter().min().unwrap();
    }
    ck
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ck = 0;
    for line in input.lines() {
        let whitespace = line.split_whitespace();
        let v: Vec<i64> = whitespace.map(|x| x.parse().unwrap()).collect();
        for i in 0..v.len() {
            for j in i+1..v.len() {
                if v[i] % v[j] == 0 {
                    ck += v[i] / v[j];
                } else if v[j] % v[i] == 0 {
                    ck += v[j] / v[i];
                }
            }
        }
    }
    ck
}
