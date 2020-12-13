use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let t = inlines.next().unwrap().parse::<usize>().unwrap();
    let b = inlines.next().unwrap().split(",").filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>();
    let mut min = 9999999999;
    let mut wait = 999999999;
    for x in b {
        if x - t % x < wait {
            min = x;
            wait = x - t%x;
        }
    }
    wait*min
}

fn modd(mut n: isize, m: isize) -> isize {
    while n < 0 {n += m}
    n
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    inlines.next();
    let b = inlines.next().unwrap().split(",").enumerate().filter_map(|(i, x)| Some((i as isize, x.parse::<isize>().ok()?))).collect::<Vec<_>>();

    let mut co = 1;
    let mut a = 0;
    for (mut e, x) in b {
        e = modd(-e, x);
        while a % x != e {
            a += co;
        }
        co *= x;
    }
    a
}
