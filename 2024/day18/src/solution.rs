use std::{cmp::Reverse, collections::*};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let coords = input.lines().map(|x| {
        let [x, y] = *x.split(',').map(|n| n.parse::<usize>().unwrap()).cv() else { unreachable!() };
        [x, y]
    }).cv();
    let mut g = Grid::new(b'.', 71, 71);

    for &[x, y] in &coords[..1024] {
        g[P(x, y)] = b'#';
    }

    let mut dist = Grid::new(-1, g.width, g.height);
    let start = P(0i64, 0);
    let end = P(70i64, 70);

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start)));
    while let Some(Reverse((cdist, p))) = q.pop() {
        if dist[p] != -1 { continue; }
        dist[p] = cdist;
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') && dist[p+d] == -1 {
                q.push(Reverse((cdist+1, p+d)));
            }
        }
    }
    dist[end]
}

pub fn ppart1(input: &str, n: usize) -> i64 {
    let coords = input.lines().map(|x| {
        let [x, y] = *x.split(',').map(|n| n.parse::<usize>().unwrap()).cv() else { unreachable!() };
        [x, y]
    }).cv();
    let mut g = Grid::new(b'.', 71, 71);

    for &[x, y] in &coords[..n] {
        g[P(x, y)] = b'#';
    }

    let mut dist = Grid::new(-1, g.width, g.height);
    let start = P(0i64, 0);
    let end = P(70i64, 70);

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start)));
    while let Some(Reverse((cdist, p))) = q.pop() {
        if dist[p] != -1 { continue; }
        dist[p] = cdist;
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') && dist[p+d] == -1 {
                q.push(Reverse((cdist+1, p+d)));
            }
        }
    }
    dist[end]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let coords = input.lines().map(|x| {
        let [x, y] = *x.split(',').map(|n| n.parse::<usize>().unwrap()).cv() else { unreachable!() };
        [x, y]
    }).cv();
    for i in 1024..coords.len() {
        if ppart1(input, i) == -1 {
            return format!("{},{}", coords[i-1][0], coords[i-1][1]);
        }
    }
    "".into()
}
