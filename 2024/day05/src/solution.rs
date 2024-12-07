use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn is_ok(g: &HashMap<i64, Vec<i64>>, ord: &[i64]) -> bool {
    let mut depc = HashMap::<i64, i64>::new();
    for (i, ch) in g {
        if !ord.contains(&i) {
            continue;
        }
        for j in ch {
            if ord.contains(j) {
                *depc.entry(*j).or_default() += 1;
            }
        }
    }

    for &n in ord {
        if depc.get(&n).unwrap_or(&0) != &0 {
            return false;
        }
        if g.contains_key(&n) {
            for &j in &g[&n] {
                *depc.get_mut(&j).unwrap_or(&mut 1) -= 1;
            }
        }
    }
    true
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ss = input.split("\n\n");
    let nums: Vec<(i64, i64)> = ss.next().unwrap().lines().map(|x| {
        let mut it = x.split("|").map(|n| n.parse::<i64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    }).cv();

    let mut g = HashMap::<i64, Vec<i64>>::new();
    for &(x, y) in &nums {
        g.entry(x).or_default().push(y);
    }


    let mut ans = 0;
    for line in ss.next().unwrap().lines() {
        let ins = line.split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        if is_ok(&g, &ins) {
            ans += ins[ins.len()/2];
        }
    }
    ans
}

fn reorder(g: &HashMap<i64, Vec<i64>>, ord: &[i64]) -> Vec<i64> {
    let mut depc = HashMap::<i64, i64>::new();
    for (i, ch) in g {
        if !ord.contains(&i) {
            continue;
        }
        for j in ch {
            if ord.contains(j) {
                *depc.entry(*j).or_default() += 1;
            }
        }
    }

    let mut q = vec![];
    for n in ord {
        if depc.get(n).unwrap_or(&0) == &0 {
            q.push(*n);
        }
    }

    let mut r = vec![];
    while let Some(i) = q.pop() {
        r.push(i);
        for ch in g.get(&i).unwrap_or(&vec![]) {
            *depc.get_mut(ch).unwrap_or(&mut 1) -= 1;
            if depc.get(ch) == Some(&0) {
                q.push(*ch);
            }
        }
    }
    r
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ss = input.split("\n\n");
    let nums: Vec<(i64, i64)> = ss.next().unwrap().lines().map(|x| {
        let mut it = x.split("|").map(|n| n.parse::<i64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    }).cv();

    let mut g = HashMap::<i64, Vec<i64>>::new();
    for &(x, y) in &nums {
        g.entry(x).or_default().push(y);
    }


    let mut ans = 0;
    for line in ss.next().unwrap().lines() {
        let ins = line.split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        if !is_ok(&g, &ins) {
            let f = reorder(&g, &ins);
            // eprintln!("input: {ins:?}");
            // eprintln!("fixed: {f:?}");
            // eprintln!();
            ans += f[f.len()/2];
        }
    }
    ans
}
