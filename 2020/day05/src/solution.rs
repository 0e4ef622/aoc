use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut max = 0;
    for line in input.lines() {
        let mut l = 0;
        let mut r = 128;
        for c in line[..7].bytes() {
            let m = (l + r)/2;
            match c {
                b'F' => r = m,
                b'B' => l = m,
                _ => (),
            }
        }
        let row = l;
        let col = i32::from_str_radix(&line[7..].replace("R","1").replace("L","0"), 2).unwrap();
        let x = 8*row + col;
        max = max.max(x);
    }
    max
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut s = vec![];
    for line in input.lines() {
        let mut l = 0;
        let mut r = 128;
        for c in line[..7].bytes() {
            let m = (l + r)/2;
            match c {
                b'F' => r = m,
                b'B' => l = m,
                _ => (),
            }
        }
        let row = l;
        let col = i32::from_str_radix(&line[7..].replace("R","1").replace("L","0"), 2).unwrap();
        let x = 8*row + col;
        s.push(x);
    }
    s.sort();
    for w in s.windows(2) {
        if w[1] - w[0] == 2 {
            return w[1] - 1;
        }
    }
    0
}
