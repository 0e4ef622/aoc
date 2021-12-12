use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

fn is_lower(current: &str) -> bool {
    current.bytes().all(|c| (b'a'..=b'z').contains(&c))
}

fn dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashMap<&'a str, u8>,
    mut double: bool,
    current: &'a str,
) -> usize {
    if current == "end" {
        return 1;
    }
    if is_lower(current) {
        let v_cnt = visited.entry(current).or_default();
        if *v_cnt >= 1 {
            if double || current == "start" {
                return 0;
            }
            double = true;
        }
        *v_cnt += 1;
    }
    let mut paths = 0;
    for &neighbor in graph.get(current).unwrap_or(&vec![]) {
        paths += dfs(graph, visited, double, neighbor);
    }

    if let Some(v_cnt) = visited.get_mut(current) {
        *v_cnt -= 1;
    }

    paths
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = HashMap::new();
    for line in input.lines() {
        let mut w = line.split('-');
        let a = w.next().unwrap();
        let b = w.next().unwrap();
        g.entry(a).or_insert(vec![]).push(b);
        g.entry(b).or_insert(vec![]).push(a);
    }
    dfs(&mut g, &mut Default::default(), true, "start")
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = HashMap::new();
    for line in input.lines() {
        let mut w = line.split('-');
        let a = w.next().unwrap();
        let b = w.next().unwrap();
        g.entry(a).or_insert(vec![]).push(b);
        g.entry(b).or_insert(vec![]).push(a);
    }
    dfs(&mut g, &mut Default::default(), false, "start")
}
