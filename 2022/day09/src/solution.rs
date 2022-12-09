use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn nm(a: (i64, i64), b: (i64, i64)) -> bool {
    (a.1-b.1).abs() > 1 || (a.0-b.0).abs() > 1
}

fn cu(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    let dx = (a.0 - b.0).signum();
    let dy = (a.1 - b.1).signum();
    (b.0+dx, b.1+dy)
}

fn mv(h: &mut (i64, i64), d: char) {
    match d {
        'R' => h.0 += 1,
        'L' => h.0 -= 1,
        'U' => h.1 += 1,
        'D' => h.1 -= 1,
        _ => (),
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut h = (0,0);
    let mut t = (0,0);
    let mut s = HashSet::new();
    s.insert(t);
    for line in input.lines() {
        let (d, n) = line.split_once(' ').unwrap();
        let n = n.parse::<i64>().unwrap();
        let d = d.chars().next().unwrap();

        for _ in 0..n {
            mv(&mut h, d);
            if nm(h, t) {
                t = cu(h, t);
            }
            s.insert(t);
        }
    }
    s.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut v = vec![(0, 0); 10];
    let mut s = HashSet::new();
    s.insert((0,0));
    for line in input.lines() {
        let (d, n) = line.split_once(' ').unwrap();
        let n = n.parse::<i64>().unwrap();
        let d = d.chars().next().unwrap();

        for _ in 0..n {
            mv(&mut v[0], d);
            for i in 1..10 {
                if nm(v[i-1], v[i]) {
                    v[i] = cu(v[i-1], v[i]);
                }
            }
            s.insert(v[9]);
            let t = v[9];
        }
    }
    s.len()
}
