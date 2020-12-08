use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().map(|x| (false, x)).collect::<Vec<_>>();
    let mut acc = 0;
    let mut i: isize = 0;
    loop {
        let (v, line) = &mut lines[i as usize];
        if *v { break acc; }
        *v = true;
        let n = line[4..].parse::<isize>().unwrap();
        match &line[..3] {
            "jmp" => i += n,
            "acc" => { acc += n; i += 1; },
            _ => i += 1,
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().map(|x| (false, x)).collect::<Vec<_>>();
    for k in 0..lines.len() {
        let mut acc = 0;
        let mut i: isize = 0;
        lines.iter_mut().for_each(|x| x.0=false);
        let r = loop {
            let (v, line) = &mut lines[i as usize];
            if *v { break false; }
            *v = true;
            let n = line[4..].parse::<isize>().unwrap();
            match (&line[..3], i == k as isize) {
                ("jmp", false) | ("nop", true) => i += n,
                ("acc", _) => { acc += n; i += 1; },
                _ => i += 1,
            }
            if i as usize == lines.len() { break true; }
        };
        if r { return acc; }
    }
    0
}
