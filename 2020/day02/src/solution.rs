use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut w = line.split(&[' ','-'][..]);
        let l: usize = w.next().unwrap().parse().unwrap();
        let r: usize = w.next().unwrap().parse().unwrap();
        let c = w.next().unwrap().as_bytes()[0];
        let p = w.next().unwrap();
        let C = p.bytes().filter(|&x| x == c).count();
        R += (C >= l && C <= r) as u32;
    }
    R
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut R = 0;
    for line in inlines {
        let mut w = line.split(&[' ','-'][..]);
        let l: usize = w.next().unwrap().parse().unwrap();
        let r: usize = w.next().unwrap().parse().unwrap();
        let c = w.next().unwrap().as_bytes()[0];
        let p = w.next().unwrap().as_bytes();

        unsafe {
            R += ((*p.get_unchecked(l-1) == c) ^ (*p.get_unchecked(r-1) == c)) as u32;
        }
    }
    R
}
