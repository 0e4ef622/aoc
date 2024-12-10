use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let mut h = Vec::<(P<i64>, P<i64>)>::new();
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if g[y][x] == b'0' {
                let p = P(x as i64, y as i64);
                h.push((p, p));
            }
        }
    }

    let mut ans = HashSet::<(P<i64>, P<i64>)>::new();
    while let Some((p, o)) = h.pop() {
        let c = g[p];
        for d in Dir::iter() {
            if g.get(p + d).copied() == Some(c+1) {
                if c+1 == b'9' {
                    ans.insert((p+d, o));
                } else {
                    h.push((p+d, o));
                }
            }
        }
    }
    ans.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let mut h = Vec::<(P<i64>, P<i64>)>::new();
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if g[y][x] == b'0' {
                let p = P(x as i64, y as i64);
                h.push((p, p));
            }
        }
    }

    let mut ans = 0;
    while let Some((p, o)) = h.pop() {
        let c = g[p];
        for d in Dir::iter() {
            if g.get(p + d).copied() == Some(c+1) {
                if c+1 == b'9' {
                    ans += 1;
                } else {
                    h.push((p+d, o));
                }
            }
        }
    }
    ans
}
