use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn mkmask(mask: &str) -> (u64, u64) {
    let mut mask1 = 0;
    let mut mask2 = 0;
    for ch in mask.bytes() {
        mask1 <<= 1;
        mask2 <<= 1;
        match ch {
            b'0' => (),
            b'1' => mask1 |= 1,
            b'X' => mask2 |= 1,
            _ => (),
        }
    }
    (mask1, mask2)
}
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mask = &inlines.next().unwrap()[7..];
    let (mut mask1, mut mask2) = mkmask(mask);
    let mut mem = HashMap::new();
    for line in inlines {
        match &line[..3] {
            "mas" => {
                let mask = mkmask(&line[7..]);
                mask1 = mask.0;
                mask2 = mask.1;
            }
            "mem" => {
                let p: (u64, u64) = s!("mem[{}] = {}" <- line).unwrap();
                *mem.entry(p.0).or_default() = (p.1 & mask2) | mask1;
            }
            _ => (),
        }
    }
    mem.values().sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut mask = &inlines.next().unwrap()[7..];
    let mut mem = HashMap::new();
    for line in inlines {
        match &line[..3] {
            "mas" => {
                mask = &line[7..];
            }
            "mem" => {
                let mut p: (u64, u64) = s!("mem[{}] = {}" <- line).unwrap();
                let mut v = vec![];
                for (i, ch) in mask.bytes().rev().enumerate() {
                    match ch {
                        b'1' => p.0 |= 1 << i,
                        b'X' => { p.0 &= !(1<<i); v.push(i); }
                        _ => (),
                    }
                }
                for i in 0..(1u64 << v.len()) {
                    let mut n = p.0;
                    v.iter().enumerate()
                        .for_each(|(g, x)| if i & (1<<g) != 0 { n |= 1<<x; });
                    *mem.entry(n).or_default() = p.1;
                }
            }
            _ => (),
        }
    }
    mem.values().sum::<u64>()
}
