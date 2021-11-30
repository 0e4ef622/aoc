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
    let input = input.trim().as_bytes();
    let mut sum = 0;
    for i in 1..input.len() {
        if input[i] == input[i-1] {
            sum += (input[i] - b'0') as usize;
        }
    }
    if input[input.len()-1] == input[0] {
        sum += (input[0] - b'0') as usize;
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.trim().as_bytes();
    let mut sum = 0;
    for i in 0..input.len() {
        if input[i] == input[(i + input.len()/2) % input.len()] {
            sum += (input[i] - b'0') as usize;
        }
    }
    sum
}
