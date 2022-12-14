use std::{collections::*, cmp::{min, max}};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn sandfs(cave: &mut HashMap<(i64, i64), char>, x: i64, y: i64, floor: i64) -> bool {
    if cave.get(&(x, y)) != None {
        return true;
    }
    if y == floor {
        return false;
    }
    if sandfs(cave, x, y+1, floor) && sandfs(cave, x-1, y+1, floor) && sandfs(cave, x+1, y+1, floor) {
        cave.insert((x, y), 'o');
        return true;
    } else {
        return false;
    }
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
    sandfs(&mut cave, 500, 0, floor);
    cave.values().filter(|&&x| x == 'o').count()
}

fn sandfs2(cave: &mut HashMap<(i64, i64), char>, x: i64, y: i64, floor: i64) -> usize{
    let mut c = 0;
    let mut stack = vec![(x, y)];
    while let Some((x, y)) = stack.pop() {
        if cave.get(&(x, y)) != None { continue; }
        if y == floor+1 {
            cave.insert((x, y), 'o');
            c += 1;
            continue;
        }

        cave.insert((x, y), 'o');
        c += 1;
        stack.push((x, y+1));
        stack.push((x-1, y+1));
        stack.push((x+1, y+1));
    }
    c
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
    sandfs2(&mut cave, 500, 0, floor)
}
