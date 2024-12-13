use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn claw(input: &str) -> i64 {
    let lines = input.lines().cv();
    let [ax, ay] = *lines[0][12..].split(", Y+").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };
    let [bx, by] = *lines[1][12..].split(", Y+").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };
    let [x, y] = *lines[2][9..].split(", Y=").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };

    let a = P(ax, ay);
    let b = P(bx, by);
    let mut ans = i64::MAX;
    for i in 0..101 {
        for j in 0..101 {
            if i*a + j*b == P(x, y) {
                ans = ans.min(3*i + j);
            }
        }
    }
    if ans == i64::MAX {
        0
    } else {
        ans
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for section in input.split("\n\n") {
        ans += claw(section);
    }
    ans
}


fn claw2(input: &str) -> Option<i64> {
    let lines = input.lines().cv();
    let [ax, ay] = *lines[0][12..].split(", Y+").map(|s| s.parse::<f64>().unwrap()).cv() else { unreachable!() };
    let [bx, by] = *lines[1][12..].split(", Y+").map(|s| s.parse::<f64>().unwrap()).cv() else { unreachable!() };
    let [px, py] = *lines[2][9..].split(", Y=").map(|s| s.parse::<f64>().unwrap()).cv() else { unreachable!() };
    let [px, py] = [px + 10000000000000.0, py + 10000000000000.0];
    let A = P(ax, ay).ai();
    let B = P(bx, by).ai();
    let X = P(px, py).ai();

    let i = (by*px - bx*py) / (ax*by - ay*bx);
    let j = (ay*px - ax*py) / (ay*bx - ax*by);
    let i = i.round() as i64;
    let j = j.round() as i64;
    if i*A + j*B == X {
        Some(3*i + j)
    } else {
        None
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for section in input.split("\n\n") {
        if let Some(r) = claw2(section) {
            ans += r;
        }
    }
    ans
}
