use std::{cmp::Reverse, collections::*};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    let mut dist = [
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
    ];

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start, Dir::R)));
    while let Some(Reverse((cdist, p, d))) = q.pop() {
        if g.get(p) != Some(&b'.') { continue; }
        if dist[d as usize][p] != -1 { continue; }
        dist[d as usize][p] = cdist;
        q.push(Reverse((cdist+1, p+d, d)));
        q.push(Reverse((cdist+1000, p, d.r())));
        q.push(Reverse((cdist+1000, p, d.l())));
    }
    dist.iter().map(|d| d[end]).min().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    let mut dist = [
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
    ];

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start, Dir::R)));
    while let Some(Reverse((cdist, p, d))) = q.pop() {
        if g.get(p) != Some(&b'.') { continue; }
        if dist[d as usize][p] != -1 { continue; }
        dist[d as usize][p] = cdist;
        q.push(Reverse((cdist+1, p+d, d)));
        q.push(Reverse((cdist+1000, p, d.r())));
        q.push(Reverse((cdist+1000, p, d.l())));
    }
    let mindist = dist.iter().map(|d| d[end]).min().unwrap();


    let mut vis = [
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
        Grid::new(vec![-1; g.width*g.height], g.width),
    ];
    vis[0][end] = 1;
    vis[1][end] = 1;
    vis[2][end] = 1;
    vis[3][end] = 1;
    let mut q = VecDeque::new();
    for d in Dir::iter() {
        if dist[d as usize][end] == mindist {
            q.push_back((end-d, d, (end, d)));
        }
    }
    while let Some((p, d, par)) = q.pop_front() {
        if g.get(p) != Some(&b'.') { continue; }
        if vis[d as usize][p] != -1 { continue; }
        vis[d as usize][p] = 0;
        if ((p+d, d) == par && dist[d as usize][p]+1 == dist[d as usize][par.0])
        || ((p, d.r()) == par && dist[d as usize][p] + 1000 == dist[d.r() as usize][par.0])
        || ((p, d.l()) == par && dist[d as usize][p] + 1000 == dist[d.l() as usize][par.0]) {
            vis[d as usize][p] = 1;
            q.push_back((p-d, d, (p, d)));
            q.push_back((p, d.r(), (p, d)));
            q.push_back((p, d.l(), (p, d)));
        }
    }

    let mut ans = 0;
    for i in 0..g.height {
        for j in 0..g.width {
            let ispath = (0..4).any(|k| vis[k][i][j] == 1);
            ans += ispath as usize;
            if ispath {
                g[i][j] = b'O';
            }
        }
    }
    g.print();
    assert_ne!(ans, 591);
    ans
}
