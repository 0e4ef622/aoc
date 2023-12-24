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

fn solve_t1(rocks: [[[f64; 3]; 2]; 3]) -> f64 {
    let [[x1, y1, z1], [vx1, vy1, vz1]] = rocks[0];
    let [[x2, y2, z2], [vx2, vy2, vz2]] = rocks[1];
    let [[x3, y3, z3], [vx3, vy3, vz3]] = rocks[2];

    let yz = y1*(z2 - z3) + y2*(-z1 + z3) + y3*(z1 - z2);
    let xz = x1*(-z2 + z3) + x2*(z1 - z3) + x3*(-z1 + z2);
    let xy = x1*(y2 - y3) + x2*(-y1 + y3) + x3*(y1 - y2);
    let vxvy = vx1*(vy2 - vy3) + vx2*(-vy1 + vy3) + vx3*(vy1 - vy2);
    let vxvz = vx1*(-vz2 + vz3) + vx2*(vz1 - vz3) + vx3*(-vz1 + vz2);
    let vyvz = vy1*(vz2 - vz3) + vy2*(-vz1 + vz3) + vy3*(vz1 - vz2);

    let n = (vx2 - vx3)*yz + (vy2 - vy3)*xz + (vz2 - vz3)*xy;
    let d = (z2 - z3)*vxvy + (y2 - y3)*vxvz + (x2 - x3)*vyvz;
    n / d
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let hs = input.lines().map(|l|
        l.split(" @ ").map(|s| s.split(", ").map(|x| x.trim().parse::<f64>().unwrap()).cv().try_into().unwrap()).cv().try_into().unwrap()
    ).take(3).cv().try_into().unwrap();
    let t1 = solve_t1(hs);
    let t2 = solve_t1([hs[1], hs[0], hs[2]]);

    let [[x1, y1, z1], [vx1, vy1, vz1]] = hs[0];
    let [[x2, y2, z2], [vx2, vy2, vz2]] = hs[1];
    let [cx1, cy1, cz1] = [
        x1 + vx1*t1,
        y1 + vy1*t1,
        z1 + vz1*t1,
    ];
    let [cx2, cy2, cz2] = [
        x2 + vx2*t2,
        y2 + vy2*t2,
        z2 + vz2*t2,
    ];
    let [vx, vy, vz] = [
        (cx2 - cx1) / (t2 - t1),
        (cy2 - cy1) / (t2 - t1),
        (cz2 - cz1) / (t2 - t1),
    ];
    let [x, y, z] = [
        x1 + vx1*t1 - vx*t1,
        y1 + vy1*t1 - vy*t1,
        z1 + vz1*t1 - vz*t1,
    ];
    x + y + z
}
