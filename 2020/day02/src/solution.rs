use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut b = line.as_bytes().iter();

        loop {
            let ch = *b.next().unwrap();
            if ch == b'-' { break; }
            l = 10*l + (ch - b'0') as usize;
        }
        loop {
            let ch = *b.next().unwrap();
            if ch == b' ' { break; }
            r = 10*r + (ch - b'0') as usize;
        }
        let c = *b.next().unwrap();
        b.next();
        b.next();
        let p = b.as_slice();

        let C = p.iter().filter(|&&x| x == c).count();
        R += (C >= l && C <= r) as u32;
    }
    R
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut b = line.as_bytes().iter();

        loop {
            let ch = *b.next().unwrap();
            if ch == b'-' { break; }
            l = 10*l + (ch - b'0') as usize;
        }
        loop {
            let ch = *b.next().unwrap();
            if ch == b' ' { break; }
            r = 10*r + (ch - b'0') as usize;
        }
        let c = *b.next().unwrap();
        b.next();
        b.next();
        let p = b.as_slice();

        unsafe {
            R += ((*p.get_unchecked(l-1) == c) != (*p.get_unchecked(r-1) == c)) as u32;
        }
    }
    R
}
