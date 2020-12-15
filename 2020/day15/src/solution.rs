use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let input: Vec<usize> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();
    let mut cnt = HashMap::new();
    input[..input.len()-1]
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i,x)| { cnt.insert(x, i); });

    let mut last = *input.last().unwrap();
    for t in input.len()..2020 {
        let oldlast = last;
        if !cnt.contains_key(&last) {
            last = 0;
        } else {
            last = t-1 - cnt[&last];
        }
        cnt.insert(oldlast, t-1);
    }
    last
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input: Vec<usize> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();
    let mut cnt = HashMap::new();
    input[..input.len()-1]
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i,x)| { cnt.insert(x, i); });

    let mut last = *input.last().unwrap();
    for t in input.len()..30000000 {
        println!("{}", last);
        let oldlast = last;
        if !cnt.contains_key(&last) {
            last = 0;
        } else {
            last = t-1 - cnt[&last];
        }
        cnt.insert(oldlast, t-1);
    }
    last
}
