use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut dist = Grid::new(-1i64, g.width, g.height);

    let mut q = VecDeque::new();
    q.push_back((0, start));
    dist[start] = 0;
    while let Some((cdist, p)) = q.pop_front() {
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') && dist[p+d] == -1 {
                dist[p+d] = cdist+1;
                q.push_back((cdist+1, p+d));
            }
        }
    }

    let mut ans = 0;
    for (p, &c) in g.iter_coords() {
        if c == b'#' {
            for fromd in Dir::iter() {
                for tod in Dir::iter() {
                    if [g.get(p+fromd), g.get(p+tod)] == [Some(&b'.'); 2] {
                        let d = dist[p+fromd] + 2 + dist[end] - dist[p+tod];
                        let saved = dist[end] - d;
                        if saved >= 100 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let start = g.find(b'S').unwrap();
    let end = g.find(b'E').unwrap();
    g[start] = b'.';
    g[end] = b'.';

    let mut dist = Grid::new(-1i64, g.width, g.height);

    let mut q = VecDeque::new();
    q.push_back((0, start));
    dist[start] = 0;
    while let Some((cdist, p)) = q.pop_front() {
        for d in Dir::iter() {
            if g.get(p+d) == Some(&b'.') && dist[p+d] == -1 {
                dist[p+d] = cdist+1;
                q.push_back((cdist+1, p+d));
            }
        }
    }

    let mut ans = 0usize;
    let mut saved_cnts = [0; 128];
    for (s, &c) in g.iter_coords() {
        for dx in -20i64..=20 {
            for dy in -20i64..=20 {
                if dx.abs()+dy.abs() > 20 { continue; }
                let e = s + P(dx, dy);
                if [g.get(s), g.get(e)] == [Some(&b'.'); 2] {
                    let d = dist[s] + dx.abs() + dy.abs() + dist[end] - dist[e];
                    let saved = dist[end] - d;
                    if saved >= 100 {
                        ans += 1;
                    }
                    // if saved >= 50 {
                    //     saved_cnts[saved as usize] += 1;
                    // }
                }
            }
        }
    }

    for (i, &c) in saved_cnts.iter().enumerate() {
        if c > 0 {
            eprintln!("{c} cheats saved {i} ps");
        }
    }
    assert_ne!(ans, 2141159);
    assert_ne!(ans, 2028034);
    ans
}
