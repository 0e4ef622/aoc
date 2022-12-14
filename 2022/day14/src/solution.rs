use std::{collections::*, cmp::{min, max}};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn add_sand(cave: &mut HashMap<(i64, i64), char>, floor: i64) -> bool {
    let (mut x, mut y) = (500, 0);
    while y < floor {
        if cave.get(&(x, y+1)) == None {
            y += 1;
        } else if cave.get(&(x-1, y+1)) == None {
            y += 1;
            x -= 1;
        } else if cave.get(&(x+1, y+1)) == None {
            y += 1;
            x += 1;
        } else {
            cave.insert((x, y), 'o');
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cave = HashMap::new();
    let mut floor = 0;
    for line in input.lines() {
        let v = line.split(" -> ").cv();
        let fp = v[0].split(",").map(|x| x.parse::<i64>().unwrap()).cv();
        let (mut x, mut y) = (fp[0], fp[1]);
        for &d in &v[1..] {
            let d = d.split(",").map(|x| x.parse::<i64>().unwrap()).cv();
            let (dx, dy) = (d[0], d[1]);
            for i in min(x, dx)..=max(x, dx) {
                for j in min(y, dy)..=max(y, dy) {
                    cave.insert((i, j), '#');
                    floor = floor.max(j);
                }
            }
            (x, y) = (dx, dy);
        }
    }
    let mut c = 0;
    while add_sand(&mut cave, floor) {
        c += 1;
    }
    c
}

fn add_sand2(cave: &mut HashMap<(i64, i64), char>, floor: i64) {
    let (mut x, mut y) = (500, 0);
    loop {
        if y == floor + 1 {
            cave.insert((x, y), 'o');
            break;
        } else if cave.get(&(x, y+1)) == None {
            y += 1;
        } else if cave.get(&(x-1, y+1)) == None {
            y += 1;
            x -= 1;
        } else if cave.get(&(x+1, y+1)) == None {
            y += 1;
            x += 1;
        } else {
            cave.insert((x, y), 'o');
            break;
        }
    }
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cave = HashMap::new();
    let mut floor = 0;
    for line in input.lines() {
        let v = line.split(" -> ").cv();
        let fp = v[0].split(",").map(|x| x.parse::<i64>().unwrap()).cv();
        let (mut x, mut y) = (fp[0], fp[1]);
        for &d in &v[1..] {
            let d = d.split(",").map(|x| x.parse::<i64>().unwrap()).cv();
            let (dx, dy) = (d[0], d[1]);
            for i in min(x, dx)..=max(x, dx) {
                for j in min(y, dy)..=max(y, dy) {
                    cave.insert((i, j), '#');
                    floor = floor.max(j);
                }
            }
            (x, y) = (dx, dy);
        }
    }
    let mut c = 0;
    while cave.get(&(500, 0)) == None {
        add_sand2(&mut cave, floor);
        c += 1;
    }
    c
}
