use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for (i, line) in inlines.enumerate() {
        let line = line.as_bytes();
        R += (line[i*3 % line.len()] == b'#') as u32;
    }
    R
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    for (i, line) in inlines.enumerate() {
        let line = line.as_bytes();
        a += (line[i % line.len()] == b'#') as u32;
        b += (line[i*3 % line.len()] == b'#') as u32;
        c += (line[i*5 % line.len()] == b'#') as u32;
        d += (line[i*7 % line.len()] == b'#') as u32;
        if (i%2==0) {
            e += (line[i/2 % line.len()] == b'#') as u32;
        }
    }
    a*b*c*d*e
}
