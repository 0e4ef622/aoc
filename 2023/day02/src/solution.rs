use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut a = 0;
    'l: for line in input.lines() {
        let s1: Vec<_> = line.split(": ").collect();
        let id = s1[0][5..].parse::<i64>().unwrap();
        let games = s1[1].split("; ").cv();
        for game in games {
            for bits in game.split(", ") {
                let co = bits.split(" ").cv();
                let n = co[0].parse::<i64>().unwrap();
                match co[1] {
                    "red" if n > 12 => continue 'l,
                    "green" if n > 13 => continue 'l,
                    "blue" if n > 14 => continue 'l,
                    _ => (),
                }
            }
        }
        a += id;
    }
    a
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut a = 0;
    for line in input.lines() {
        let s1: Vec<_> = line.split(": ").collect();
        let id = s1[0][5..].parse::<i64>().unwrap();
        let bags = s1[1].split("; ").cv();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for bag in bags {
            for bits in bag.split(", ") {
                let co = bits.split(" ").cv();
                let n = co[0].parse::<i64>().unwrap();
                match co[1] {
                    "red" => r = r.max(n),
                    "green" => g = g.max(n),
                    "blue" => b = b.max(n),
                    _ => (),
                }
            }
        }
        a += r*g*b;
    }
    a
}
