use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

fn re(ing: &[[i64; 5]], props: [i64; 5], amt: i64, part2: bool) -> i64 {
    if ing.len() == 0 || amt == 0 {
        if amt != 0 || ing.len() != 0 {
            0
        } else if part2 == false {
            props[..4].iter().map(|&x| x.max(0)).product()
        } else if props[4] == 500 {
            props[..4].iter().map(|&x| x.max(0)).product()
        } else {
            0
        }
    } else {
        (0..=amt)
            .map(|x| {
                re(
                    &ing[1..],
                    [
                        props[0] + x * ing[0][0],
                        props[1] + x * ing[0][1],
                        props[2] + x * ing[0][2],
                        props[3] + x * ing[0][3],
                        props[4] + x * ing[0][4],
                    ],
                    amt - x,
                    part2,
                )
            })
            .max()
            .unwrap()
    }
}

fn parse(input: &str) -> Vec<[i64; 5]> {
    input
        .lines()
        .map(|line| {
            let bits: [i64; 5] = line
                .split(", ")
                .map(|b| {
                    b.split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                })
                .cv()
                .try_into()
                .unwrap();
            bits
        })
        .cv()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let ing = parse(input);
    re(&ing, [0; 5], 100, false)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let ing = parse(input);
    re(&ing, [0; 5], 100, true)
}
