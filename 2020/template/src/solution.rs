use std::collections::*;
use rand::random;
use serde_scan::scan as s;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

pub fn part1(input: &str) -> impl std::fmt::Display {
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
