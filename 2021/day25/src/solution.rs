use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn mv(w: usize, h: usize, d: &mut HashSet<(usize,usize)>,r: &mut HashSet<(usize,usize)>) -> bool {
    let mut b = false;
    let mut nd = HashSet::new();
    let mut nr = HashSet::new();
    for &(i,j) in &*r {
        if d.contains(&(i,(j+1)%w)) || r.contains(&(i, (j+1)%w)) {
            nr.insert((i,j));
        } else {
            nr.insert((i,(j+1)%w));
            b=true;
        }
    }
    *r = nr;
    for &(i,j) in &*d {
        if d.contains(&((i+1)%h,j)) || r.contains(&((i+1)%h, j)) {
            nd.insert((i,j));
        } else {
            nd.insert(((i+1)%h,j));
            b=true;
        }
    }

    *d = nd;
    b
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = input.trim().lines().cv();
    let w = g[0].len();
    let h = g.len();
    let mut d = HashSet::new();
    let mut r = HashSet::new();
    for (i, l) in g.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            if c == b'>' {
                r.insert((i, j));
            } else if c == b'v' {
                d.insert((i, j));
            }

        }
    }
    (0..).take_while(|_| mv(w,h,&mut d,&mut r)).count() + 1
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
