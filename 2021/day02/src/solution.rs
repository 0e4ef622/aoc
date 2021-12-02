use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut depth = 0;
    let mut pos = 0;
    for line in input.lines() {
        let mut w = line.split_whitespace();
        let d = w.next().unwrap();
        let s = w.next().unwrap().parse::<i64>().unwrap();
        match d {
            "forward" => pos += s,
            "down" => depth += s,
            _ => depth -= s,
        }
    }
    depth*pos
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut w = line.split_whitespace();
        let d = w.next().unwrap();
        let s = w.next().unwrap().parse::<i64>().unwrap();
        match d {
            "forward" => {pos += s; depth += aim*s; }
            "down" => aim += s,
            _ => aim -= s,
        }
    }
    depth*pos
}
