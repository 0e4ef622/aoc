use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

const W: i64 = 101;
const H: i64 = 103;
// const W: i64 = 11;
// const H: i64 = 7;
const W2: i64 = W/2;
const H2: i64 = H/2;
const W21: i64 = W/2+1;
const H21: i64 = H/2+1;

fn quadrant(p: P<i64>) -> usize {
    match (p.0.rem_euclid(W), p.1.rem_euclid(H)) {
        (0..W2, 0..H2) => 0,
        (W21..W, 0..H2) => 1,
        (W21..W, H21..H) => 2,
        (0..W2, H21..H) => 3,
        _ => 4,
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let bots = input.lines().map(|l| {
        let [px, py, _, _, vx, vy] = *l[2..].split([',', ' ', 'v', '=']).cv() else { unreachable!() };
        (P(px.parse::<i64>().unwrap(), py.parse::<i64>().unwrap()), P(vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap()))
    }).cv();

    let mut qc = [0; 5];
    for b in bots {
        let p = b.0 + 100*b.1;
        qc[quadrant(p)] += 1;
    }
    let ans = qc[0] * qc[1] * qc[2] * qc[3];
    assert_ne!(ans, 226650624);
    ans
}

fn longest_run(g: &[[bool; W as usize]; H as usize]) -> usize {
    let mut re = 0;
    for r in g {
        let mut cr = 0;
        for &c in r {
            if c {
                cr += 1;
            } else {
                re = re.max(cr);
                cr = 0;
            }
        }
        re = re.max(cr);
    }
    re
}

pub fn print(bots: &[(P<i64>, P<i64>)], t: i64) -> bool {
    let mut g = [[false; W as usize]; H as usize];
    for b in bots {
        let p = b.0 + (t % (W*H))*b.1;
        g[p.1.rem_euclid(H) as usize][p.0.rem_euclid(W) as usize] = true;
    }
    if longest_run(&g) >= 10 {
        for r in g {
            for c in r {
                if c {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        true
    } else {
        false
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let bots = input.lines().map(|l| {
        let [px, py, _, _, vx, vy] = *l[2..].split([',', ' ', 'v', '=']).cv() else { unreachable!() };
        (P(px.parse::<i64>().unwrap(), py.parse::<i64>().unwrap()), P(vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap()))
    }).cv();

    for i in 0..W*H {
        // println!("{i}");
        if print(&bots, i) {
            return i;
        }
        // println!();
        // std::thread::sleep_ms(50);
    }
    0
}
