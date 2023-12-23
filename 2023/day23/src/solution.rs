use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

use Dir::*;

fn dfs(p: P<i64>, pre: P<i64>, g: &mut Grid<u8>, w: usize) -> i64 {
    if p.au() == P(w-2, g.height-1) {
        return 0;
    }

    let d = p-pre;
    match g[p] {
        b'>' if d != P(1, 0) => return i64::MIN,
        b'v' if d != P(0, 1) => return i64::MIN,
        b'<' if d != P(-1, 0) => return i64::MIN,
        b'^' if d != P(0, -1) => return i64::MIN,
        _ => (),
    }

    let orig = g[p.au()];
    g[p.au()] = b'#';
    let mut maxd = i64::MIN;
    for d in [U,D,L,R] {
        let pp = p + d;
        if pp.0 < 0 || pp.0 >= w as i64 || pp.1 < 0 || pp.1 >= g.height as i64 || g[pp] == b'#' {
            continue;
        }
        maxd = maxd.max(1 + dfs(pp, p, g, w));
    }
    g[p] = orig;
    maxd
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::new(input.as_bytes().to_vec(), 142);
    dfs(P(1, 0), P(1, 0), &mut g, 141)
}

fn reduce(p: P<i64>, mut i: i32, mut w: i32, g: &mut Grid<u8>, gr: &mut Vec<Vec<(i32, i32)>>) {
    if g[p] > b'.' {
        let j = (g[p] - b'/') as i32;
        gr[i as usize].push((j, w));
        gr[j as usize].push((i, w));
        return;
    }
    let adjc = Dir::iter().filter(|&d| !matches!(g.get(p+d), Some(b'#'))).count();
    if adjc > 2 {
        let oi = i;
        let ow = w;
        i = gr.len() as i32;
        w = 0;
        g[p] = b'/' + i as u8;
        gr.push(vec![]);
        reduce(p, oi, ow, g, gr);
    } else {
        g[p] = b'%';
    }
    for d in Dir::iter() {
        let Some(c) = g.get(p+d) else { continue };
        if !b"#%".contains(&g[p+d]) && g[p+d] != b'/'+i as u8 {
            reduce(p+d, i, w+1, g, gr);
        }
    }
}

fn dfs2(i: usize, v: &mut [bool], g: &mut Vec<Vec<(i32, i32)>>) -> i64 {
    if i == 1 {
        return 0;
    }
    v[i] = true;
    let mut r = i64::MIN;
    for j in 0..g[i].len() {
        let (c, w) = g[i][j];
        if !v[c as usize] {
            r = r.max(w as i64 + dfs2(c as _, v, g));
        }
    }
    v[i] = false;
    r
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);
    for c in b"<>v^" {
        g.replace(c, b'.');
    }
    let w = g.width;
    let h = g.height;
    let mut gr = vec![vec![]; 2];
    g[0][1] = b'/' + 0;
    g[h-1][w-2] = b'/' + 1;
    reduce(P(1, 1), 0, 1, &mut g, &mut gr);
    dfs2(0, &mut vec![false; gr.len()], &mut gr)
}
