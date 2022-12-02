use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sc = 0;
    for line in input.lines() {
        let mut wh = line.split_whitespace();
        let a = wh.next().unwrap();
        let x = wh.next().unwrap();
        let a = match a {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => unreachable!(),
        };
        let x = match x {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => unreachable!(),
        };
        sc += x+1;
        if a == x {
            sc += 3;
        } else if x == (3 + a+1)%3 {
            sc += 6;
        }
    }
    sc
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sc = 0;
    for line in input.lines() {
        let mut wh = line.split_whitespace();
        let a = wh.next().unwrap();
        let x = wh.next().unwrap();
        let a = match a {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => unreachable!(),
        };
        let x = match x {
            "X" => sc += (a+2)%3+1,
            "Y" => sc += 3 + a%3+1,
            "Z" => sc += (a+1)%3 + 6+1,
            _ => unreachable!(),
        };
    }
    sc
}
