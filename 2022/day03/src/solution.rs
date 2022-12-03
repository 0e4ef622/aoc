use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let l = line.len();
        let le: BTreeSet<_> = line[..l/2].chars().collect();
        let ri: BTreeSet<_> = line[l/2..].chars().collect();
        let x = le.intersection(&ri).next().unwrap();
        let pr = if ('a'..='z').contains(x) {
            1 + (*x as i32 - 'a' as i32)
        } else {
            27 + (*x as i32 - 'A' as i32)
        };
        ans += pr;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for mut chunk in &input.lines().chunks(3) {
        let a: BTreeSet<_> = chunk.next().unwrap().chars().collect();
        let b: BTreeSet<_> = chunk.next().unwrap().chars().collect();
        let c: BTreeSet<_> = chunk.next().unwrap().chars().collect();
        let x = a.intersection(&b).copied().collect::<BTreeSet<_>>();
        let x = x.intersection(&c).next().unwrap();
        let pr = if ('a'..='z').contains(x) {
            1 + (*x as i32 - 'a' as i32)
        } else {
            27 + (*x as i32 - 'A' as i32)
        };
        ans += pr;
    }
    ans
}
