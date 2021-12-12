use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

fn is_lower(current: &str) -> bool {
    current.bytes().all(|c| (b'a'..=b'z').contains(&c))
}

fn dfs<'a>(
    graph: &Vec<Vec<u8>>,
    memo: &mut HashMap<(u32, bool, u8), usize>,
    mut visited: u32,
    mut double: bool,
    current: u8,
) -> usize {
    if current == 2 {
        return 1;
    }
    if let Some(&c) = memo.get(&(visited, double, current)) {
        return c;
    }

    let pd = double;
    let pv = visited;
    if current % 2 == 0 {
        if visited & (1 << current) != 0 {
            if double || current == 0 {
                return 0;
            }
            double = true;
        }
        visited |= 1 << current;
    }
    let mut paths = 0;
    for &neighbor in &graph[current as usize] {
        paths += dfs(graph, memo, visited, double, neighbor);
    }

    memo.insert((pv, pd, current), paths);

    paths
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    // Even IDs represent small caves, odd represent large.
    let mut id_map = HashMap::new();
    id_map.insert("start", 0);
    id_map.insert("end", 2);
    let mut g: Vec<Vec<u8>> = vec![vec![]; 3];
    let mut x = 2;
    let mut id = |g: &mut Vec<_>, s| {
        match id_map.entry(s) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                x += (x % 2 != is_lower(s) as u8) as u8 + 1;
                g.resize((x+1) as usize, vec![]);
                *e.insert(x)
            }
        }
    };
    for line in input.lines() {
        let mut w = line.split('-');
        let a = id(&mut g, w.next().unwrap());
        let b = id(&mut g, w.next().unwrap());
        g[a as usize].push(b);
        g[b as usize].push(a);
    }
    dfs(&g, &mut Default::default(), 0, true, 0)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    // Even IDs represent small caves, odd represent large.
    let mut id_map = HashMap::new();
    id_map.insert("start", 0);
    id_map.insert("end", 2);
    let mut g: Vec<Vec<u8>> = vec![vec![]; 3];
    let mut x = 2;
    let mut id = |g: &mut Vec<_>, s| {
        match id_map.entry(s) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                x += (x % 2 != is_lower(s) as u8) as u8 + 1;
                g.resize((x+1) as usize, vec![]);
                *e.insert(x)
            }
        }
    };
    for line in input.lines() {
        let mut w = line.split('-');
        let a = id(&mut g, w.next().unwrap());
        let b = id(&mut g, w.next().unwrap());
        g[a as usize].push(b);
        g[b as usize].push(a);
    }
    dfs(&g, &mut Default::default(), 0, false, 0)
}
