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

pub fn part2_og(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let mut h = Vec::<P<i64>>::new();
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if g[y][x] == b'0' {
                let p = P(x as i64, y as i64);
                h.push(p);
            }
        }
    }

    let mut ans = 0;
    while let Some(p) = h.pop() {
        let c = g[p];
        for d in Dir::iter() {
            if g.get(p + d).copied() == Some(c+1) {
                if c+1 == b'9' {
                    ans += 1;
                } else {
                    h.push(p+d);
                }
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let mut h = vec![
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
        Vec::<P<i64>>::with_capacity(384),
    ];
    for (y, r) in g.iter_mut().enumerate() {
        for (x, c) in r.iter_mut().enumerate() {
            h[(*c - b'0') as usize].push(P(x as i64, y as i64));
            *c = (*c == b'0') as u8;
        }
    }

    let mut g2 = Grid::from_parts(vec![0; g.width*g.height], g.width);
    for w in h.windows(2) {
        g2.array.fill(0);
        let pre = &w[0];
        let v = &w[1];
        for &p in v {
            for d in Dir::iter() {
                if let Some(&v) = g.get(p+d) {
                    g2[p] += v;
                }
            }
        }
        for &p in pre {
            g[p] = 0;
        }
        (g, g2) = (g2, g);
    }
    h[9].iter().map(|p| g[*p] as u32).sum::<u32>()
}
