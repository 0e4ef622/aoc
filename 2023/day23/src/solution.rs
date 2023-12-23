use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

use Dir::*;

fn dfs(p: P<i64>, pre: P<i64>, g: &mut Grid<u8>, p2: bool) -> i64 {
    if p.au() == P(g.width-2, g.height-1) {
        return 0;
    } else if p.0 < 0 || p.0 >= g.width as i64 || p.1 < 0 || p.1 >= g.height as i64 || b"#O".contains(&g[p.au()]) {
        return i64::MIN;
    }

    if !p2 {
        let d = p-pre;
        match g[p.au()] {
            b'>' if d != P(1, 0) => return i64::MIN,
            b'v' if d != P(0, 1) => return i64::MIN,
            b'<' if d != P(-1, 0) => return i64::MIN,
            b'^' if d != P(0, -1) => return i64::MIN,
            _ => (),
        }
    }

    let orig = g[p.au()];
    g[p.au()] = b'O';
    let mut maxd = i64::MIN;
    for d in [U,D,L,R] {
        let pp = p + d.p();
        if pp == pre { continue; }
        maxd = maxd.max(1 + dfs(pp, p, g, p2));
    }
    g[p.au()] = orig;


    maxd
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    dfs(P(1, 0), P(1, 0), &mut g, false)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    dfs(P(1, 0), P(1, 0), &mut g, true)
}
