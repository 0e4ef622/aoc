use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let h = lines.len();
    let w = lines[0].len();
    let mut map = vec![vec![9; w+2]; h+2];
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            map[y+1][x+1] = (c-b'0') as i64;
        }
    }

    let mut cn = 0;
    for (i, j) in iproduct!(1..=h, 1..=w) {
        let l = map[i][j-1];
        let r = map[i][j+1];
        let u = map[i-1][j];
        let d = map[i+1][j];
        let c = map[i][j];
        if c < l && c < r && c < u && c < d {
            cn+=1+c;
        }
    }
    cn
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let h = lines.len();
    let w = lines[0].len();
    let mut map = vec![vec![9; w+2]; h+2];
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            map[y+1][x+1] = (c-b'0') as i64;
        }
    }

    let mut ds = Dsu::new((w+2)*(h+2));
    let ix = |i, j| i*(w+2) + j;

    for (i, j) in iproduct!(1..=h, 1..=w) {
        let l = (map[i][j-1], ix(i, j-1));
        let r = (map[i][j+1], ix(i, j+1));
        let u = (map[i-1][j], ix(i-1, j));
        let d = (map[i+1][j], ix(i+1, j));
        let c = map[i][j];
        if c == 9 { continue; }
        if l.0 != 9 { ds.merge(ix(i,j), l.1); }
        if r.0 != 9 { ds.merge(ix(i,j), r.1); }
        if u.0 != 9 { ds.merge(ix(i,j), u.1); }
        if d.0 != 9 { ds.merge(ix(i,j), d.1); }
    }

    let mut v = vec![];
    for i in 0..ds.s.len() {
        if ds.p[i] == i { v.push(ds.s[i]); }
    }
    v.sort();
    v[v.len()-3..].iter().product::<usize>()
}
