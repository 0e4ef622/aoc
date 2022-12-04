use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cnt = 0;
    for line in input.lines() {
        let mut s = line.split(',');
        let mut ss = s.next().unwrap().split('-');
        let a = ss.next().unwrap().parse::<i64>().unwrap();
        let b = ss.next().unwrap().parse::<i64>().unwrap();

        let mut ss = s.next().unwrap().split('-');
        let c = ss.next().unwrap().parse::<i64>().unwrap();
        let d = ss.next().unwrap().parse::<i64>().unwrap();

        if a<=c && b>=d || c<=a && d>=b {
            cnt += 1;
        }
    }
    cnt
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cnt = 0;
    for line in input.lines() {
        let mut s = line.split(',');
        let mut ss = s.next().unwrap().split('-');
        let a = ss.next().unwrap().parse::<i64>().unwrap();
        let b = ss.next().unwrap().parse::<i64>().unwrap();

        let mut ss = s.next().unwrap().split('-');
        let c = ss.next().unwrap().parse::<i64>().unwrap();
        let d = ss.next().unwrap().parse::<i64>().unwrap();

        let e = a.max(c);
        let f = d.min(b);
        if e <= f {
            cnt += 1;
        }
    }
    cnt
}
