use std::collections::*;
use itertools::{iproduct, Itertools};

pub trait CollectVec: Iterator + Sized {
    fn cv(self) -> Vec<Self::Item> {
        self.collect()
    }
    fn cs(self) -> BTreeSet<Self::Item> where Self::Item: Ord {
        self.collect()
    }
}
impl<I: Iterator> CollectVec for I {}

fn neighbors(c: u8, x: usize, y: usize) -> [[usize; 2]; 2] {
    match c {
        b'|' => [[x, y+1], [x, y-1]],
        b'-' => [[x+1, y], [x-1, y]],
        b'L' => [[x, y-1], [x+1, y]],
        b'J' => [[x, y-1], [x-1, y]],
        b'7' => [[x, y+1], [x-1, y]],
        b'F' => [[x, y+1], [x+1, y]],
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|x| x.as_bytes()).cv();
    let h = grid.len();
    let w = grid[0].len();
    let mut s = [0, 0];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'S' {
                s = [j, i];
                break;
            }
        }
    }

    let [sx, sy] = s;
    let mut c = [0, 0];
    let mut p = s;
    if sy < h-1 && neighbors(grid[sy+1][sx], sx, sy+1).contains(&s) { c = [sx, sy+1]; }
    else if sy > 0 && neighbors(grid[sy-1][sx], sx, sy-1).contains(&s) { c = [sx, sy-1]; }
    else if sx < w-1 && neighbors(grid[sy][sx+1], sx+1, sy).contains(&s) { c = [sx+1, sy]; }
    else if sx > 0 && neighbors(grid[sy][sx-1], sx-1, sy).contains(&s) { c = [sx-1, sy]; }

    let mut len = 1;
    while c != [sx, sy] {
        len += 1;
        let ne = neighbors(grid[c[1]][c[0]], c[0], c[1]);
        if p != ne[0] {
            p = c;
            c = ne[0];
        } else {
            p = c;
            c = ne[1];
        }
    }
    len/2
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|x| x.as_bytes()).cv();
    let h = grid.len();
    let w = grid[0].len();
    let mut s = [0, 0];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'S' {
                s = [j, i];
                break;
            }
        }
    }

    let [sx, sy] = s;
    let mut c = [0, 0];
    let mut p = s;
    if sy < h-1 && neighbors(grid[sy+1][sx], sx, sy+1).contains(&s) { c = [sx, sy+1]; }
    else if sy > 0 && neighbors(grid[sy-1][sx], sx, sy-1).contains(&s) { c = [sx, sy-1]; }
    else if sx < w-1 && neighbors(grid[sy][sx+1], sx+1, sy).contains(&s) { c = [sx+1, sy]; }
    else if sx > 0 && neighbors(grid[sy][sx-1], sx-1, sy).contains(&s) { c = [sx-1, sy]; }

    let mut len = 1;
    let mut area = 0;
    while c != [sx, sy] {
        len += 1;
        area += c[0] as i64*p[1] as i64 - p[0] as i64*c[1] as i64;
        let ne = neighbors(grid[c[1]][c[0]], c[0], c[1]);
        if p != ne[0] {
            p = c;
            c = ne[0];
        } else {
            p = c;
            c = ne[1];
        }
    }
    area += sx as i64*p[1] as i64 - sy as i64*p[0] as i64;
    area.abs()/2 + 1 - len/2
}
