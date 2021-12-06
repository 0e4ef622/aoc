use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut n: Vec<i64> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    for _ in 0..80 {
        // eprintln!("{:?}", n);
        for j in 0..n.len() {
            if n[j] == 0 {
                n.push(8);
                n[j] = 7;
            }
            n[j] -= 1;
        }
    }

    n.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let m: Vec<i64> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut n = [0; 9];
    for v in m {
        n[v as usize] += 1;
    }

    for _ in 0..256 {
        n.rotate_left(1);
        n[6] += n[8];
    }

    n.iter().sum::<u64>()
}
