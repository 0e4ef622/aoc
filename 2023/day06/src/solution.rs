use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let times = lines[0].split_ascii_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).cv();
    let dists = lines[1].split_ascii_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).cv();
    let mut ans = 1;
    for i in 0..times.len() {
        let t = times[i];
        let d = dists[i];
        let mut c = 0;
        for hold in 0..=t {
            let dd = hold*(t-hold);
            if dd > d {
                c += 1;
            }
        }
        ans *= c;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let time = lines[0].split_ascii_whitespace().skip(1).cv().join("").parse::<i64>().unwrap();
    let dist = lines[1].split_ascii_whitespace().skip(1).cv().join("").parse::<i64>().unwrap();
    // hold*time - hold*hold = dist
    // 0 = dist - hold*time + hold*hold
    // a = 1, b = -time, c = dist
    let a = 1 as f64;
    let b = -time as f64;
    let c = dist as f64;
    let t1 = (-b + (b*b - 4.*a*c).sqrt()) / (2.*a);
    let t2 = (-b - (b*b - 4.*a*c).sqrt()) / (2.*a);
    (t1-1.).ceil() - (t2+1.).floor() + 1.
}
