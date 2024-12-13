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


fn eea(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let mut x1 @ mut y1 = 0;
    let d = eea(b, a%b, &mut x1, &mut y1);
    *x = y1;
    *y = x1 - y1 * (a/b);
    return d;
}

fn find_sol(a: i64, b: i64, c: i64, x0: &mut i64, y0: &mut i64, g: &mut i64) -> bool {
    *g = eea(a, b, x0, y0);
    if (c%*g != 0) {
        return false;
    }

    *x0 *= c/ *g;
    *y0 *= c/ *g;
    return true;
}

fn dothing(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    let g = gcd(a, b);
    if c%g != 0 {
        return None;
    }

    let a = a/g;
    let b = b/g;
    let c = c/g;

    let mut i @ mut j @ mut g = 0;
    if !find_sol(a, b, c, &mut i, &mut j, &mut g) {
        return None;
    }
    Some((i, j, b, -a))
}

fn claw2(input: &str) -> Option<i64> {
    let lines = input.lines().cv();
    let [ax, ay] = *lines[0][12..].split(", Y+").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };
    let [bx, by] = *lines[1][12..].split(", Y+").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };
    let [x, y] = *lines[2][9..].split(", Y=").map(|s| s.parse::<i64>().unwrap()).cv() else { unreachable!() };
    let A = P(ax, ay);
    let B = P(bx, by);
    let X = P(x + 10000000000000, y + 10000000000000);

    let s0 = dothing(A.0, B.0, X.0)?;
    let s1 = dothing(A.1, B.1, X.1)?;
    let s0 = (s0.0 as f64, s0.1 as f64, s0.2 as f64, s0.3 as f64);
    let s1 = (s1.0 as f64, s1.1 as f64, s1.2 as f64, s1.3 as f64);

    // vx(y - y0) = vy(x - x0)
    // y = vy/vx(x - x0) + y0
    // vy0/vx0(x - x0) + y0 = vy1/vx1(x - x1) + y1
    // vy0/vx0(x - x0) - vy1/vx1(x) = vy1/vx1(-x1) + y1 - y0
    // vy0/vx0(x) - vy1/vx1(x) = vy0/vx0(x0) - vy1/vx1(x1) + y1 - y0
    // x(vy0/vx0 - vy1/vx1) = vy0/vx0(x0) - vy1/vx1(x1) + y1 - y0
    // x = (vy0/vx0(x0) - vy1/vx1(x1) + y1 - y0) / (vy0/vx0 - vy1/vx1)

    let n = s0.3*s0.0/s0.2 - s1.3*s1.0/s1.2 + s1.1 - s0.1;
    let d = s0.3/s0.2 - s1.3/s1.2;
    let x = n/d;
    let y = s0.3*(x - s0.0)/s0.2 + s0.1;
    let i = x.round() as i64;
    let j = y.round() as i64;
    if i*A + j*B == X {
        Some(3*x.round() as i64 + y.round() as i64)
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
