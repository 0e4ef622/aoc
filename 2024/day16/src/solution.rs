use std::{cmp::Reverse, collections::*};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    let mut dist = Grid::new([-1; 4], g.width, g.height);

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start, Dir::R)));
    while let Some(Reverse((cdist, p, d))) = q.pop() {
        if dist[p][d as usize] != -1 { continue; }
        dist[p][d as usize] = cdist;
        if g[p+d] != b'#' && dist[p+d][d as usize] == -1 { q.push(Reverse((cdist+1, p+d, d))); }
        if dist[p][d.r() as usize] == -1 { q.push(Reverse((cdist+1000, p, d.r()))); }
        if dist[p][d.l() as usize] == -1 { q.push(Reverse((cdist+1000, p, d.l()))); }
    }
    *dist[end].iter().min().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    let mut dist = Grid::new([-1; 4], g.width, g.height);

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start, Dir::R)));
    while let Some(Reverse((cdist, p, d))) = q.pop() {
        if dist[p][d as usize] != -1 { continue; }
        dist[p][d as usize] = cdist;
        if g[p+d] != b'#' && dist[p+d][d as usize] == -1 { q.push(Reverse((cdist+1, p+d, d))); }
        if dist[p][d.r() as usize] == -1 { q.push(Reverse((cdist+1000, p, d.r()))); }
        if dist[p][d.l() as usize] == -1 { q.push(Reverse((cdist+1000, p, d.l()))); }
    }
    let mindist = *dist[end].iter().min().unwrap();


    let mut vis = Grid::new([-1; 4], g.width, g.height);
    vis[end] = [1; 4];
    let mut q = VecDeque::new();
    for d in Dir::iter() {
        if dist[end][d as usize] == mindist {
            q.push_back((end-d, d, (end, d)));
        }
    }
    while let Some((p, d, par)) = q.pop_front() {
        if vis[p][d as usize] != -1 { continue; }
        vis[p][d as usize] = 0;
        if ((p+d, d) == par && dist[p][d as usize]+1 == dist[par.0][d as usize])
        || ((p, d.r()) == par && dist[p][d as usize] + 1000 == dist[par.0][d.r() as usize])
        || ((p, d.l()) == par && dist[p][d as usize] + 1000 == dist[par.0][d.l() as usize]) {
            vis[p][d as usize] = 1;
            if g[p-d] != b'#' { q.push_back((p-d, d, (p, d))); }
            q.push_back((p, d.r(), (p, d)));
            q.push_back((p, d.l(), (p, d)));
        }
    }

    let mut ans = 0;
    for i in 0..g.height {
        for j in 0..g.width {
            let ispath = vis[i][j].contains(&1);
            ans += ispath as usize;
        }
    }
    ans
}
