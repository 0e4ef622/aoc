use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut x = 0;
    for line in input.lines() {
        let dg = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
        let a = (dg[0] as i64 - '0' as i64);
        let b = (*dg.last().unwrap() as i64 - '0' as i64);
        x += a*10 + b;
    }
    x
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut x = 0;
    for line in input.lines() {
        let mut nums = vec![];
        let mut i = 0;
        while i < line.len() {
            if line[i..].starts_with("one")   { nums.push(1); }
            else if line[i..].starts_with("two")   { nums.push(2); }
            else if line[i..].starts_with("three") { nums.push(3); }
            else if line[i..].starts_with("four")  { nums.push(4); }
            else if line[i..].starts_with("five")  { nums.push(5); }
            else if line[i..].starts_with("six")   { nums.push(6); }
            else if line[i..].starts_with("seven") { nums.push(7); }
            else if line[i..].starts_with("eight") { nums.push(8); }
            else if line[i..].starts_with("nine")  { nums.push(9); }
            else if line[i..].starts_with("1") { nums.push(1); }
            else if line[i..].starts_with("2") { nums.push(2); }
            else if line[i..].starts_with("3") { nums.push(3); }
            else if line[i..].starts_with("4") { nums.push(4); }
            else if line[i..].starts_with("5") { nums.push(5); }
            else if line[i..].starts_with("6") { nums.push(6); }
            else if line[i..].starts_with("7") { nums.push(7); }
            else if line[i..].starts_with("8") { nums.push(8); }
            else if line[i..].starts_with("9") { nums.push(9); }
            i += 1;
        }
        x += nums[0]*10 + nums.last().unwrap();
    }
    x
}
