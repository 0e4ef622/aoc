use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut w = line.split_whitespace();
        let mut r = w.next().unwrap().split("-").map(|x| x.parse().unwrap());
        let l: usize = r.next().unwrap();
        let r: usize = r.next().unwrap();
        let c = &w.next().unwrap().chars().next().unwrap();
        let p = w.next().unwrap();
        let C = p.chars().filter(|x| x == c).count();
        if C >= l && C <= r {
            R += 1;
        }
    }
    R
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut w = line.split_whitespace();
        let mut r = w.next().unwrap().split("-").map(|x| x.parse().unwrap());
        let l: usize = r.next().unwrap();
        let r: usize = r.next().unwrap();
        let c = &w.next().unwrap()[0..1];
        let p = w.next().unwrap();

        if (&p[l-1..l] == c) ^ (&p[r-1..r] == c) {
            R += 1;
        }
    }
    R
}
