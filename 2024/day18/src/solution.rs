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

pub fn part2(input: &str) -> impl std::fmt::Display {
    let coords = input.lines().map(|x| {
        let [x, y] = *x.split(',').map(|n| n.parse::<usize>().unwrap()).cv() else { unreachable!() };
        [x, y]
    }).cv();
    let mut g = Grid::new(b'.', 71, 71);
    for &[x, y] in &coords {
        g[P(x, y)] = b'#';
    }

    let mut dsu = Dsu::new(71*71);
    let m = |p: P<i64>| (p.1*71+p.0) as usize;
    for (p, &c) in g.iter_coords() {
        if c == b'#' { continue; }
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') {
                dsu.merge(m(p), m(p+d));
            }
        }
    }

    for (i, &[x, y]) in coords.iter().enumerate().rev() {
        let p = P(x, y).ai();
        g[P(x, y)] = b'.';
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') {
                dsu.merge(m(p), m(p+d));
            }
        }
        if dsu.find(0) == dsu.find(71*71-1) {
            return format!("{x},{y}");
        }
    }
    unreachable!();
}
