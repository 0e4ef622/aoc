use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    input.as_bytes().windows(4).enumerate().filter(|x| x.1.iter().collect::<BTreeSet<_>>().len() == 4).next().unwrap().0 + 4
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input.as_bytes().windows(14).enumerate().filter(|x| x.1.iter().collect::<BTreeSet<_>>().len() == 14).next().unwrap().0 + 14
}
