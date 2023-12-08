use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let nodes: HashMap<_, _> = lines[2..].iter().map(|l| {
        let name = &l[0..3];
        let left = &l[7..10];
        let right = &l[12..15];
        (name, (left, right))
    }).collect();
    let ins = lines[0].trim().as_bytes();
    let mut cur = "AAA";
    let mut cnt = 0;
    while cur != "ZZZ" {
        cur = if ins[cnt%ins.len()] == b'L' {
            nodes[cur].0
        } else {
            nodes[cur].1
        };
        cnt += 1;
    }
    cnt
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().cv();
    let nodes: HashMap<_, _> = lines[2..].iter().map(|l| {
        let name = &l[0..3];
        let left = &l[7..10];
        let right = &l[12..15];
        (name, (left, right))
    }).collect();
    let ins = lines[0].trim().as_bytes();
    let mut starts = nodes.keys().copied().filter(|x| x.ends_with('A')).cv();
    let stuff = starts.iter().copied().map(|start| {
        let mut cur = start;
        let mut v = HashMap::<&str, _>::new();
        let mut cnt = 0;
        while !v.contains_key(cur) {
            v.insert(cur, cnt);
            cur = if ins[cnt%ins.len()] == b'L' {
                nodes[cur].0
            } else {
                nodes[cur].1
            };
            cnt += 1;
        }
        let period = cnt - v[cur];
        let pcnt = cnt;
        period
    }).cv();
    let mut ans = 1;
    for m in stuff {
        ans = m*ans;
    }
    ans*ins.len()
}
