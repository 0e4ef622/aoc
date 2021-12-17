use std::{collections::*, ops::RangeInclusive};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn shoot(mut vx: i32, mut vy: i32, tx: RangeInclusive<i32>, ty: RangeInclusive<i32>) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut my = 0;

    loop {
        x += vx;
        y += vy;
        my = y.max(my);
        vx -= vx.signum();
        vy -= 1;

        if y < *ty.start() {
            return i32::MIN;
        }

        if tx.contains(&x) && ty.contains(&y) {
            return my;
        }
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let t = input[13..].trim().split(", ").flat_map(|s| s[2..].split("..")).map(|s| s.parse::<i32>().unwrap()).cv();
    let mut my = 0;
    for x in 0..100 {
        for y in 0..200 {
            my = my.max(shoot(x, y, t[0]..=t[1], t[2]..=t[3]));
        }
    }
    my
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let t = input[13..].trim().split(", ").flat_map(|s| s[2..].split("..")).map(|s| s.parse::<i32>().unwrap()).cv();
    let mut c = 0;
    for x in 0..500 {
        for y in -400..300 {
            c += (shoot(x, y, t[0]..=t[1], t[2]..=t[3]) != i32::MIN) as u64;
        }
    }
    c
}
