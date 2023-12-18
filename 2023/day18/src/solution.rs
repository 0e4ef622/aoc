use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Dir {
    U,D,L,R
}
fn mv(x: i32, y: i32, d: Dir) -> (i32, i32) {
    match d {
        Dir::U => (x, y-1),
        Dir::D => (x, y+1),
        Dir::L => (x-1, y),
        Dir::R => (x+1, y),
    }
}

fn mv2(x: i64, y: i64, n: i64, d: Dir) -> (i64, i64) {
    match d {
        Dir::U => (x, y-n),
        Dir::D => (x, y+n),
        Dir::L => (x-n, y),
        Dir::R => (x+n, y),
    }
}

use Dir::*;

impl Dir {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => U,
            "D" => D,
            "L" => L,
            "R" => R,
            _ => unreachable!(),
        }
    }
}

fn fill(g: &mut [[i64; 1000]; 1000], x: usize, y: usize) {
    g[y][x] = 1;
    if g[y][x-1] == 0 { fill(g, x-1, y); }
    if g[y][x+1] == 0 { fill(g, x+1, y); }
    if g[y-1][x] == 0 { fill(g, x, y-1); }
    if g[y+1][x] == 0 { fill(g, x, y+1); }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut x = 500;
    let mut y = 500;
    let mut g = [[0; 1000]; 1000];
    g[500][500] = 1;
    for line in input.lines() {
        let w = line.split_ascii_whitespace().cv();
        let dir = Dir::from_str(w[0]);
        let c = w[1].parse::<i64>().unwrap();
        for i in 0..c {
            (x, y) = mv(x, y, dir);
            g[y as usize][x as usize] = 1;
        }
    }

    fill(&mut g, 501, 501);
    g.iter().map(|x| x.into_iter().sum::<i64>()).sum::<i64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut x = 0;
    let mut y = 0;
    let mut area = 0;
    let mut e = 0;
    for line in input.lines() {
        let w = line.split_ascii_whitespace().cv();
        let dir = [R,D,L,U][usize::from_str_radix(&w[2][7..8], 16).unwrap()];
        let c = i64::from_str_radix(&w[2][2..][..5], 16).unwrap();
        e += c;
        let (x2, y2) = mv2(x, y, c, dir);
        area += x*y2 - x2*y;
        (x, y) = (x2, y2)
    }
    area.abs()/2 + e/2 + 1
}
