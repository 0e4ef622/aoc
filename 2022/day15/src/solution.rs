use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let yy = 2000000;
    let mut ybeacons = HashSet::new();
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let wtf = line.split(['=', ',', ':']).cv();
        let sx = wtf[1].parse::<i64>().unwrap();
        let sy = wtf[3].parse::<i64>().unwrap();
        let bx = wtf[5].parse::<i64>().unwrap();
        let by = wtf[7].parse::<i64>().unwrap();

        if by == yy {
            ybeacons.insert(bx);
        }

        let r = (sx - bx).abs() + (sy-by).abs();
        sensors.push((sx, sy, r));
        beacons.insert((bx, by));
    }

    let mut ans = 0;
    let mut iv: BTreeMap<i64, i64> = BTreeMap::new();
    for &(x, y, r) in &sensors {
        let d = r - (y-yy).abs();
        if d < 0 { continue; }
        *iv.entry(x-d).or_default() += 1;
        *iv.entry(x+d+1).or_default() -= 1;
    }
    let mut iv = iv.into_iter().cv();
    let mut c = iv[0].1;
    for w in iv.windows(2) {
        if c > 0 {
            ans += w[1].0 - w[0].0;
        }
        c += w[1].1;
    }

    ans - ybeacons.len() as i64
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let wtf = line.split(['=', ',', ':']).cv();
        let sx = wtf[1].parse::<i64>().unwrap();
        let sy = wtf[3].parse::<i64>().unwrap();
        let bx = wtf[5].parse::<i64>().unwrap();
        let by = wtf[7].parse::<i64>().unwrap();

        let r = (sx - bx).abs() + (sy-by).abs();
        sensors.push((sx, sy, r));
        beacons.insert((bx, by));
    }

    for yy in 0..=4000000 {
        let mut iv: BTreeMap<i64, i64> = BTreeMap::new();
        for &(x, y, r) in &sensors {
            let d = r - (y-yy).abs();
            if d < 0 { continue; }
            *iv.entry(x-d).or_default() += 1;
            *iv.entry(x+d+1).or_default() -= 1;
        }
        let mut c = 0;
        let mut lk = -1;
        for (&k, &v) in &iv {
            c += v;
            if c == 0 && lk == -1 {
                lk = k;
            }
        }
        if lk != *iv.iter().last().unwrap().0 {
            return lk*4000000+yy;
        }
    }

    0
}
