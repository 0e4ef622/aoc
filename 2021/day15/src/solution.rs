use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(|l| l.bytes().map(|b| (b-b'0') as i64).cv()).cv();
    let w = g[0].len();
    let h = g.len();

    let mut v = vec![vec![false; w]; h];
    let mut q = BinaryHeap::new();
    q.push((0,0,0));
    loop {
        let (r, i, j) = q.pop().unwrap();
        let r = -r;
        if i == h-1 && j == w-1 {
            return r;
        }
        if v[i][j] { continue; }
        v[i][j] = true;

        if i > 0 { q.push((-r - g[i-1][j], i-1, j)); }
        if i < h-1 { q.push((-r - g[i+1][j], i+1, j)); }
        if j > 0 { q.push((-r - g[i][j-1], i, j-1)); }
        if j < w-1 { q.push((-r - g[i][j+1], i, j+1)); }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = input.lines().map(|l| l.bytes().map(|b| (b-b'0') as i64).cv()).cv();
    let w = g[0].len();
    let h = g.len();

    let cost = |r: usize, c| (g[r%h][c%w] + (r/h) as i64 + (c/w) as i64 - 1) % 9 + 1;

    let w = w*5;
    let h = h*5;

    let mut v = vec![vec![false; w]; h];
    let mut q = BinaryHeap::new();
    q.push((0,0,0));
    loop {
        let (r, i, j) = q.pop().unwrap();
        let r = -r;
        if i == h-1 && j == w-1 {
            return r;
        }
        if v[i][j] { continue; }
        v[i][j] = true;

        if i > 0 { q.push((-r - cost(i-1, j), i-1, j)); }
        if i < h-1 { q.push((-r - cost(i+1, j), i+1, j)); }
        if j > 0 { q.push((-r - cost(i, j-1), i, j-1)); }
        if j < w-1 { q.push((-r - cost(i, j+1), i, j+1)); }
    }
}
