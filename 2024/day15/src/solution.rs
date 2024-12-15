use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn push(g: &mut Grid<u8>, p: P<i64>, d: Dir, commit: bool) -> P<i64> {
    if g[p + d] == b'#' {
        return p;
    } else if g[p+d] == b'O'
    || (g[p+d] == b'[' || g[p+d] == b']') && d.is_horizontal() {
        if push(g, p+d, d, commit) == p+d {
            return p;
        }
    } else if g[p+d] == b'[' || g[p+d] == b']' {
        let p2 = match g[p+d] {
            b'[' => p+d+Dir::R,
            b']' => p+d+Dir::L,
            _ => unreachable!(),
        };
        if push(g, p+d, d, false) == p+d || push(g, p2, d, false) == p2 {
            return p;
        } else {
            if commit {
                push(g, p+d, d, true);
                push(g, p2, d, true);
                g[p+d] = g[p];
                g[p] = b'.';
            }
            return p + d;
        }
    }
    if commit {
        g[p+d] = g[p];
        g[p] = b'.';
    }
    p + d
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let ss = input.split("\n\n").cv();
    let mut g = Grid::from_ascii(ss[0]);
    let mut p = g.find(b'@').unwrap();

    for ins in ss[1].lines().flat_map(str::bytes) {
        let d = match ins {
            b'^' => Dir::U,
            b'v' => Dir::D,
            b'<' => Dir::L,
            b'>' => Dir::R,
            _ => unreachable!("{}", ins),
        };
        p = push(&mut g, p, d, true);
    }
    let mut ans = 0;
    for (p, &c) in g.iter_coords() {
        if c == b'O' {
            ans += 100*p.1 + p.0;
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let ss = input.split("\n\n").cv();
    let mut g = Grid::from_ascii(
        &ss[0]
            .replace("#", "##")
            .replace(".", "..")
            .replace("O", "[]")
            .replace("@", "@.")
    );
    let mut p = g.find(b'@').unwrap();

    for ins in ss[1].lines().flat_map(str::bytes) {
        let d = match ins {
            b'^' => Dir::U,
            b'v' => Dir::D,
            b'<' => Dir::L,
            b'>' => Dir::R,
            _ => unreachable!("{}", ins),
        };
        p = push(&mut g, p, d, true);
    }
    let mut ans = 0;
    for (p, &c) in g.iter_coords() {
        if c == b'[' {
            ans += 100*p.1 + p.0;
        }
    }
    ans
}
