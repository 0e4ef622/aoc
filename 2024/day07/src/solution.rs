use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn tst(n: &[usize], e: usize, acc: usize) -> bool {
    if n.len() == 0 {
        return acc == e;
    }
    if acc > e {
        return false;
    }
    if tst(&n[1..], e, acc+n[0]) { return true; }
    if tst(&n[1..], e, acc*n[0]) { return true; }
    false
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let s1 = line.split(": ").cv();
        let e = s1[0].parse::<usize>().unwrap();
        let n = s1[1].split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).cv();
        if tst(&n[1..], e, n[0]) {
            ans += e;
        }
    }
    ans
}

fn ndig(n: usize) -> u32{
    if n < 10 { 1 }
    else if n < 100 { 2 }
    else if n < 1000 { 3 }
    else if n < 10000 { 4 }
    else if n < 100000 { 5 }
    else if n < 1000000 { 6 }
    else if n < 10000000 { 7 }
    else if n < 100000000 { 8 }
    else if n < 1000000000 { 9 }
    else if n < 10000000000 { 10 }
    else if n < 100000000000 { 11 }
    else if n < 1000000000000 { 12 }
    else if n < 10000000000000 { 13 }
    else if n < 100000000000000 { 14 }
    else if n < 1000000000000000 { 15 }
    else if n < 10000000000000000 { 16 }
    else if n < 100000000000000000 { 17 }
    else if n < 1000000000000000000 { 18 }
    else { 19 }
}

fn tst2(n: &[usize], e: usize, acc: usize) -> bool {
    if n.len() == 0 {
        return acc == e;
    }
    if acc > e {
        return false;
    }
    if tst2(&n[1..], e, acc+n[0]) { return true; }
    if tst2(&n[1..], e, acc*n[0]) { return true; }
    if tst2(&n[1..], e, acc*10usize.pow(ndig(n[0])) + n[0]) { return true; }
    false
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let s1 = line.split(": ").cv();
        let e = s1[0].parse::<usize>().unwrap();
        let n = s1[1].split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).cv();
        if tst2(&n[1..], e, n[0]) {
            ans += e;
        }
    }
    ans
}
