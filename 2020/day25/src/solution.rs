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
    let mut inlines = input.lines();
    let a = inlines.next().unwrap().parse::<usize>().unwrap();
    let b = inlines.next().unwrap().parse::<usize>().unwrap();
    let mut A = 0;
    let mut x = 1;
    while a != x {
        x = x*7 % 20201227;
        A+=1;
    }
    x = 1;
    let mut B = 0;
    while b != x {
        x = x*7 % 20201227;
        B+=1;
    }
    x = 1;
    for _ in 0..B {
        x = x*a % 20201227;
    }
    x
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    "click the button"
}
