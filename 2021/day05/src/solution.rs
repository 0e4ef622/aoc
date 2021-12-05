use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut s = line.split(" -> ");
        let mut a = s.next().unwrap().split(",");
        let mut b = s.next().unwrap().split(",");

        let mut ax = a.next().unwrap().parse::<i64>().unwrap();
        let mut ay = a.next().unwrap().parse::<i64>().unwrap();
        let bx = b.next().unwrap().parse::<i64>().unwrap();
        let by = b.next().unwrap().parse::<i64>().unwrap();
        if !(ax == bx || ay == by) { continue; }
        let dx = (bx - ax).signum();
        let dy = (by - ay).signum();
        *map.entry((ax, ay)).or_insert(0) += 1;
        while ax != bx || ay != by {
            ax += dx;
            ay += dy;
            *map.entry((ax, ay)).or_insert(0) += 1;
        }
    }
    map.values().filter(|x| **x>1).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut s = line.split(" -> ");
        let mut a = s.next().unwrap().split(",");
        let mut b = s.next().unwrap().split(",");

        let mut ax = a.next().unwrap().parse::<i64>().unwrap();
        let mut ay = a.next().unwrap().parse::<i64>().unwrap();
        let bx = b.next().unwrap().parse::<i64>().unwrap();
        let by = b.next().unwrap().parse::<i64>().unwrap();
        let dx = (bx - ax).signum();
        let dy = (by - ay).signum();
        *map.entry((ax, ay)).or_insert(0) += 1;
        while ax != bx || ay != by {
            ax += dx;
            ay += dy;
            *map.entry((ax, ay)).or_insert(0) += 1;
        }
    }
    map.values().filter(|x| **x>1).count()
}
