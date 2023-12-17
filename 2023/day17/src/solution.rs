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
    let mut v = vec![false; (w*h*4*4) as usize];
    let bck = |(x,y)| x>=0 && x<w && y>=0 && y<h;
    let mut ans = i32::MAX;
    while let Some((mut d, (x, y), c, dir)) = q.pop() {
        d = -d;
        if v[(dir as i32 + 4*(c + 4*(x + w*y))) as usize] { continue; }
        v[(dir as i32 + 4*(c + 4*(x + w*y))) as usize] = true;
        d += (g[y as usize][x as usize] - b'0') as i32;
        if x == w-1 && y == h-1 {
            ans = d;
            break;
        }
        let r = mv(x,y,right(dir));
        let l = mv(x,y,left(dir));
        let f = mv(x,y,dir);
        if bck(r) && !v[(right(dir) as i32 + 4*(1 + 4*(r.0 + w*r.1))) as usize] { q.push((-d, r, 1, right(dir))); }
        if bck(l) && !v[(left(dir) as i32 + 4*(1 + 4*(l.0 + w*l.1))) as usize] { q.push((-d, l, 1, left(dir))); }
        if c < 3 && bck(f) && !v[(dir as i32 + 4*(c+1 + 4*(f.0 + w*f.1))) as usize] { q.push((-d, f, c+1, dir)); }
    }
    ans - (g[0][0] - b'0') as i32
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(str::as_bytes).cv();
    let w = g[0].len() as i32;
    let h = g.len() as i32;
    let mut q = BinaryHeap::new();
    q.push((0i32, (0i32, 0i32), 0, R));
    let mut v = vec![false; (w*h*4*11) as usize];
    let bck = |(x,y)| x>=0 && x<w && y>=0 && y<h;
    let mut ans = i32::MAX;
    let mut ac = 0;
    while let Some((mut d, (x, y), c, dir)) = q.pop() {
        d = -d;
        if v[(dir as i32 + 4*(c + 11*(x + w*y))) as usize] { continue; }
        v[(dir as i32 + 4*(c + 11*(x + w*y))) as usize] = true;
        d += (g[y as usize][x as usize] - b'0') as i32;
        if x == w-1 && y == h-1 {
            ans = d;
            break;
        }
        let r = mv(x,y,right(dir));
        let l = mv(x,y,left(dir));
        let f = mv(x,y,dir);
        if c >= 4 && bck(r) && !v[(right(dir) as i32 + 4*(1 + 11*(r.0 + w*r.1))) as usize] { q.push((-d, r, 1, right(dir))); }
        if c >= 4 && bck(l) && !v[(left(dir) as i32 + 4*(1 + 11*(l.0 + w*l.1))) as usize] { q.push((-d, l, 1, left(dir))); }
        if c < 10 && bck(f) && !v[(dir as i32 + 4*(c+1 + 11*(f.0 + w*f.1))) as usize] { q.push((-d, f, c+1, dir)); }
    }
    ans - (g[0][0] - b'0') as i32
}

