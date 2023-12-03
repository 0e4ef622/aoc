use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn is_symbol(c: u8) -> bool {
    !(b'0'..=b'9').contains(&c) && c != b'.'
}

fn get_numbers2(s: &str) -> impl Iterator<Item = (usize, usize, i64)> + '_ {
    s
        .split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty())
        .map(|n| (n.as_ptr() as usize - s.as_ptr() as usize, n.len(), n.parse::<i64>().unwrap()))
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().cv();
    let mut g = HashMap::new();
    let mut numbers = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            g.insert((i as i64, j as i64), c);
        }
        numbers.extend(get_numbers2(row).map(|(j, c, n)| (i as i64,j as i64,c as i64,n)));
    }
    let mut ans = 0;
    'num: for (i, j, c, n) in numbers {
        for di in -1..=1 {
            for dj in -1..=c {
                let Some(&adj) = g.get(&(i+di, j+dj)) else { continue; };
                if is_symbol(adj) {
                    ans += n;
                    continue 'num;
                }
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().cv();
    let mut g = HashMap::new();
    let mut numbers = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            g.insert((i as i64, j as i64), c);
        }
        numbers.extend(get_numbers2(row).map(|(j, c, n)| (i as i64,j as i64,c as i64,n)));
    }
    let mut adjcnt = HashMap::new();
    'num: for (i, j, c, n) in numbers {
        for di in -1..=1 {
            for dj in -1..=c {
                let Some(&adj) = g.get(&(i+di, j+dj)) else { continue; };
                if adj == b'*' {
                    adjcnt.entry((i+di, j+dj)).or_insert(vec![]).push(n);
                }
            }
        }
    }
    let mut ans = 0;
    for v in adjcnt.values() {
        if v.len() == 2 {
            ans += v[0]*v[1];
        }
    }
    ans
}
