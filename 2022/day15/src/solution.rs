use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
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

    let mut set = HashSet::new();
    // let yy = 10;
    let yy = 2000000;
    for &(x, y, r) in &sensors {
        let d = r - (y-yy).abs();
        for x in x-d..=x+d {
            if !beacons.contains(&(x, yy)) {
                set.insert(x);
            }
        }
    }

    set.len()
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
            dbg!(lk, yy);
            return lk*4000000+yy;
        }
    }

    0
}
