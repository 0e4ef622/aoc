use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn is_x(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> bool {
    let [ap, av] = &a[..] else { unreachable!() };
    let [bp, bv] = &b[..] else { unreachable!() };

    let am = av[1] / av[0];
    let bm = bv[1] / bv[0];
    // y-y0 = m(x-x0)
    let ab = ap[1]-am*ap[0];
    let bb = bp[1]-bm*bp[0];
    if av[1] == 0. || bv[1] == 0. || av[0] == 0. || bv[0] == 0. {
        unreachable!();
    }
    if (am-bm).abs() < 1e-9 {
        return false;
    }

    //amx + ab = bmx + bb
    //(ab - bb) / (bm - am) = x
    let x = (ab - bb) / (bm - am);
    let y = am*x + ab;
    200000000000000. <= x && x <= 400000000000000. &&
    200000000000000. <= y && y <= 400000000000000. &&
    // 7. <= x && x <= 27. &&
    // 7. <= y && y <= 27. &&
    (x - ap[0]).signum() == av[0].signum() &&
    (y - ap[1]).signum() == av[1].signum() &&
    (x - bp[0]).signum() == bv[0].signum() &&
    (y - bp[1]).signum() == bv[1].signum()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let hs = input.lines().map(|l| l.split(" @ ").map(|s| s.split(", ").map(|x| x.trim().parse::<f64>().unwrap()).cv()).cv()).cv();
    let mut ans = 0;
    for i in 0..hs.len() {
        for j in i+1..hs.len() {
            if is_x(&hs[i], &hs[j]) {
                ans += 1;
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut hs = input.lines().map(|l| l.split(" @ ").map(|s| s.split(", ").map(|x| x.trim().parse::<f64>().unwrap()).cv()).cv()).take(3).cv();
    0
}
