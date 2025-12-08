use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut coords = vec![];
    for line in input.lines() {
        let p = line
            .trim()
            .split(',')
            .map(|e| e.parse::<i64>().unwrap())
            .cv();
        coords.push((p[0], p[1], p[2]));
    }

    let mut ds = Dsu::new(coords.len());

    let mut edges = vec![];
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            edges.push((i, j));
        }
    }
    let (edges1000, ..) = edges.select_nth_unstable_by_key(1000, |&(i, j)| {
        let a = coords[i];
        let b = coords[j];
        let d = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
        let dist2 = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
        dist2
    });

    for &mut (i, j) in edges1000 {
        ds.merge(i, j);
    }

    let mut sizes =
        ds.s.iter()
            .zip(ds.p)
            .enumerate()
            .filter_map(|(i, (s, p))| (p == i).then_some(*s))
            .cv();
    let sizes_len = sizes.len();
    let (.., last3) = sizes.select_nth_unstable(sizes_len - 4);
    last3[0] * last3[1] * last3[2]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut coords = vec![];
    for line in input.lines() {
        let p = line
            .trim()
            .split(',')
            .map(|e| e.parse::<i64>().unwrap())
            .cv();
        coords.push((p[0], p[1], p[2]));
    }

    let mut ds = Dsu::new(coords.len());

    let mut edges = vec![];
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            edges.push((i, j));
        }
    }
    edges.sort_unstable_by_key(|&(i, j)| {
        let a = coords[i];
        let b = coords[j];
        let d = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
        let dist2 = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
        dist2
    });

    let mut conns = 0;
    for &(i, j) in &edges {
        if ds.merge(i, j) {
            conns += 1;
            if conns == coords.len() - 1 {
                return coords[i].0 * coords[j].0;
            }
        }
    }

    unreachable!()
}
