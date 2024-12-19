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

    let mut vis = Grid::new(false, g.width, g.height);
    let start = P(0i64, 0);
    let end = P(70i64, 70);

    let mut q = VecDeque::new();
    q.push_back((0, start));
    vis[start] = true;
    while let Some((cdist, p)) = q.pop_front() {
        if p == end {
            return cdist;
        }
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') && !vis[p+d] {
                vis[p+d] = true;
                q.push_back((cdist+1, p+d));
            }
        }
    }
    0
}

#[derive(Debug)]
pub struct Dsu {
    pub p: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }
        Dsu { p }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        }
        let v = self.find(self.p[i]);
        self.p[i] = v;
        return self.p[i];
    }

    pub fn merge(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return false;
        }
        self.p[i] = j;
        true
    }
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
