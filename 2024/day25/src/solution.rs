use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut locks = vec![];
    let mut keys = vec![];
    for sec in input.split("\n\n") {
        let g = sec.lines().map(|x| x.bytes().cv()).cv();
        if g[0].iter().all(|&x| x == b'#') {
            // lock
            let h = g.transpose().iter().map(|x| x.iter().filter(|&&b| b == b'#').count()).cv();
            locks.push(h);
        } else {
            // key
            let h = g.transpose().iter().map(|x| x.iter().filter(|&&b| b == b'#').count()).cv();
            keys.push(h);
        };
    }

    let mut ans = 0;
    for l in &locks {
        'a: for k in &keys {
            for (a, b) in l.iter().zip(k) {
                if a+b > 7 {
                    continue 'a;
                }
            }
            ans += 1;
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
