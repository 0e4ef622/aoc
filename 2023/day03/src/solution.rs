use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn is_symbol(c: u8) -> bool {
    !(b'0'..=b'9').contains(&c) && c != b'.'
}

fn get_numbers(ro: i64, s: &str) -> Vec<(i64, i64, i64, i64)> {
    let mut r = vec![];
    let mut n = 0;
    let mut l = 0;
    let mut x = 0;
    for (i, c) in s.bytes().enumerate() {
        let i = i as i64;
        if (b'0'..=b'9').contains(&c) {
            n = n*10 + (c-b'0') as i64;
            l += 1;
        } else {
            if l != 0 {
                r.push((ro, x, l, n));
            }
            n = 0;
            x = i+1;
            l = 0;
        }
    }
    if l != 0 {
        r.push((ro, x, l, n));
    }
    r
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().cv();
    let mut g = HashMap::new();
    let mut numbers = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            g.insert((i as i64, j as i64), c);
        }
        numbers.extend(get_numbers(i as i64, row));
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
        numbers.extend(get_numbers(i as i64, row));
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
