use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn dfs<'a>(g: &HashMap<&'a str, Vec<&'a str>>, v: &mut HashMap<&'a str, u8>, mut wtf: bool, i: &'a str) -> usize {
    if i == "end" { return 1; }
    if i.to_lowercase() == i {
        let e = v.entry(i).or_default();
        if *e >= 1 {
            if wtf || i == "start" { return 0; }
            wtf = true;
        }
        *e += 1;
    }
    let mut c = 0;
    for &ch in g.get(i).unwrap_or(&vec![]) {
        c += dfs(g, v, wtf, ch);
    }

    if let Some(v) = v.get_mut(i) { *v -= 1; }

    c
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = HashMap::new();
    for line in input.lines() {
        let w = line.split('-').cv();
        let a = w[0];
        let b = w[1];
        g.entry(a).or_insert(vec![]).push(b);
        g.entry(b).or_insert(vec![]).push(a);
    }
    let mut v = HashMap::new();
    dfs(&mut g, &mut v, true, "start")
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = HashMap::new();
    for line in input.lines() {
        let w = line.split('-').cv();
        let a = w[0];
        let b = w[1];
        g.entry(a).or_insert(vec![]).push(b);
        g.entry(b).or_insert(vec![]).push(a);
    }
    let mut v = HashMap::new();
    dfs(&mut g, &mut v, false, "start")
}
