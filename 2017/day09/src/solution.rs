use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut a = 0i64;
    let mut s = 0i64;
    let mut g = false;
    let mut ex = false;
    for c in input.bytes() {
        if g {
            if c == b'!' && !ex {
                ex = true;
            } else if c == b'>' && !ex {
                g = false;
            } else {
                ex = false;
            }
            continue;
        }
        match c {
            b'{' => s += 1,
            b'}' => { a += s; s -= 1; }
            b'<' => g = true,
            _ => (),
        }
    }
    a
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut a = 0i64;
    let mut g = false;
    let mut ex = false;
    for c in input.bytes() {
        if g {
            if c == b'!' && !ex {
                ex = true;
            } else if c == b'>' && !ex {
                g = false;
            } else {
                if !ex {
                    a += 1;
                }
                ex = false;
            }
            continue;
        }
        match c {
            b'<' => g = true,
            _ => (),
        }
    }
    a
}
