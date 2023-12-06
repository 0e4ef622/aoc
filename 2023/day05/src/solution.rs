use std::{collections::*, mem::swap, ops::Range};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let secs = input.split("\n\n").cv();
    let mut seeds = secs[0][7..].split(" ").map(|x| x.parse::<i64>().unwrap()).cv();
    let mut seeds2 = vec![];
    for sec in &secs[1..] {
        for line in sec.lines().skip(1) {
            let [ds, ss, l] = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .cv()[..] else { unreachable!() };
            seeds.retain(|&s| {
                if (ss..ss+l).contains(&s) {
                    // println!("{s} -> {}",s-ss+ds);
                    seeds2.push(s-ss+ds);
                    false
                } else {
                    true
                }
            });
        }
        // println!();
        seeds.extend(seeds2.drain(..));
    }
    *seeds.iter().min().unwrap()
}

pub fn intersect(a: Range<i64>, b: Range<i64>) -> [Range<i64>; 3] {
    let ix = a.start.max(b.start)..a.end.min(b.end);
    let l = b.start..ix.start;
    let r = ix.end..b.end;
    [l, ix, r]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let secs = input.split("\n\n").cv();
    let mut seeds = secs[0][7..].split(" ").map(|x| x.parse::<i64>().unwrap()).cv().chunks(2).map(|x| x[0]..x[0]+x[1]).cv();
    let mut seeds2 = vec![];
    for sec in &secs[1..] {
        for line in sec.lines().skip(1) {
            let [ds, ss, l] = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .cv()[..] else { unreachable!() };
            let mut i = 0;
            while i < seeds.len() {
                let s = seeds[i].clone();
                let [le, ix, ri] = intersect(ss..ss+l, s);
                if !ix.is_empty() {
                    seeds2.push(ix.start-ss+ds .. ix.end-ss+ds);
                    seeds.swap_remove(i);
                    if !le.is_empty() {
                        seeds.push(le);
                    }
                    if !ri.is_empty() {
                        seeds.push(ri);
                    }
                } else {
                    i += 1;
                }
            }
        }
        seeds.extend(seeds2.drain(..));
    }
    seeds.iter().map(|r| r.start).min().unwrap()
}
