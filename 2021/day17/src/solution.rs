use std::{collections::*, ops::RangeInclusive};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn shoot(mut vx: i32, mut vy: i32, tx: RangeInclusive<i32>, ty: RangeInclusive<i32>) -> bool {
    let (mut x, mut y) = (0, 0);

    loop {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;

        if y < *ty.start() || x > *tx.end() || (x == 0 && x < *tx.start()) {
            return false;
        }

        if tx.contains(&x) && ty.contains(&y) {
            return true;
        }
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let y = input[13..].trim().split(", ").skip(1).flat_map(|s| s[2..].split("..")).map(|s| s.parse::<i32>().unwrap()).next().unwrap();
    y*(y+1)/2
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let t = input[13..].trim().split(", ").flat_map(|s| s[2..].split("..")).map(|s| s.parse::<i32>().unwrap()).cv();
    let mut c = 0;
    for x in 0..=t[1] {
        for y in t[2]..-t[2] {
            c += (shoot(x, y, t[0]..=t[1], t[2]..=t[3])) as u32;
        }
    }
    c
}
