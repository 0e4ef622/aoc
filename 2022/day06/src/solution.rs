use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    input.as_bytes().windows(4).position(|x| x.iter().cs().len() == 4).unwrap() + 4
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input.as_bytes().windows(14).position(|x| x.iter().cs().len() == 14).unwrap() + 14
}
