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
    edges.sort_unstable_by_key(|&(i, j)| {
        let a = coords[i];
        let b = coords[j];
        let d = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
        let dist2 = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
        dist2
    });

    for &(i, j) in &edges[..1000] {
        ds.merge(i, j);
    }

    let mut sizes = ds.s.clone();
    sizes.sort_unstable();
    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
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
