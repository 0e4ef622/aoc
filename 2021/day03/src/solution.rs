use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cnt = vec![0; 12];
    let tot = input.lines().count();
    for line in input.lines() {
        for (i, c) in line.bytes().enumerate() {
            if c == b'1' {
                cnt[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut eps = 0;
    for v in cnt {
        gamma <<= 1;
        eps <<= 1;
        if v > tot/2 {
            gamma |= 1;
        } else {
            eps |= 1;
        }
    }
    gamma*eps
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ox = input.lines().cv();
    let mut co2 = input.lines().cv();
    for i in 0..12 {
        if ox.len() > 1 {
            let olen = ox.len();
            let cnt = ox.iter().filter(|s| s.as_bytes()[i] == b'1').count();
            ox.retain(|s| s.as_bytes()[i] == if 2*cnt >= olen { b'1' } else { b'0' });
        }
        if co2.len() > 1 {
            let clen = co2.len();
            let cnt = co2.iter().filter(|s| s.as_bytes()[i] == b'1').count();
            co2.retain(|s| s.as_bytes()[i] == if 2*cnt < clen { b'1' } else { b'0' });
        }
    }
    usize::from_str_radix(ox[0], 2).unwrap()*usize::from_str_radix(co2[0], 2).unwrap()
}
