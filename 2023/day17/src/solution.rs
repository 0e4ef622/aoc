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
fn left(d: Dir) -> Dir {
    match d {
        Dir::U => L,
        Dir::D => R,
        Dir::L => D,
        Dir::R => U,
    }
}
fn right(d: Dir) -> Dir {
    match d {
        Dir::U => R,
        Dir::D => L,
        Dir::L => U,
        Dir::R => D,
    }
}
use Dir::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(str::as_bytes).cv();
    let w = g[0].len() as i32;
    let h = g.len() as i32;
    let mut q = BinaryHeap::new();
    q.push((0i32, (0i32, 0i32), 0, R));
    let mut v = HashSet::new();
    let bck = |(x,y)| x>=0 && x<w && y>=0 && y<h;
    let mut ans = i32::MAX;
    while let Some((mut d, (x, y), c, dir)) = q.pop() {
        d = -d;
        if !v.insert((x,y,c,dir)) { continue; }
        d += (g[y as usize][x as usize] - b'0') as i32;
        if x == w-1 && y == h-1 {
            ans = ans.min(d);
        }
        if bck(mv(x,y,right(dir))) { q.push((-d, mv(x,y,right(dir)), 1, right(dir))); }
        if bck(mv(x,y,left(dir))) { q.push((-d, mv(x,y,left(dir)), 1, left(dir))); }
        if c < 3 && bck(mv(x,y,dir)) { q.push((-d, mv(x,y,dir), c+1, dir)); }
    }
    ans - (g[0][0] - b'0') as i32
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(str::as_bytes).cv();
    let w = g[0].len() as i32;
    let h = g.len() as i32;
    let mut q = BinaryHeap::new();
    q.push((0i32, (0i32, 0i32), 0, R));
    let mut v = HashSet::new();
    let bck = |(x,y)| x>=0 && x<w && y>=0 && y<h;
    let mut ans = i32::MAX;
    while let Some((mut d, (x, y), c, dir)) = q.pop() {
        d = -d;
        if !v.insert((x,y,c,dir)) { continue; }
        d += (g[y as usize][x as usize] - b'0') as i32;
        if x == w-1 && y == h-1 {
            ans = ans.min(d);
        }
        if c >= 4 && bck(mv(x,y,right(dir))) { q.push((-d, mv(x,y,right(dir)), 1, right(dir))); }
        if c >= 4 && bck(mv(x,y,left(dir))) { q.push((-d, mv(x,y,left(dir)), 1, left(dir))); }
        if c < 10 && bck(mv(x,y,dir)) { q.push((-d, mv(x,y,dir), c+1, dir)); }
    }
    ans - (g[0][0] - b'0') as i32
}
