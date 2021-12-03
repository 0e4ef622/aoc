use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.lines().cv();
    let cnt = (0..input[0].len())
        .map(|i| input.iter().filter(|s| s.as_bytes()[i] == b'1').count())
        .cv();
    let mut gamma = 0;
    for v in cnt {
        gamma <<= 1;
        if 2*v > input.len() {
            gamma |= 1;
        }
    }
    gamma*(gamma ^ ((1<<input[0].len())-1))
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ox = input.lines().cv();
    let mut co2 = ox.clone();
    for i in 0..ox[0].len() {
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
    let o = usize::from_str_radix(ox[0], 2).unwrap();
    let c = usize::from_str_radix(co2[0], 2).unwrap();
    o*c
}
