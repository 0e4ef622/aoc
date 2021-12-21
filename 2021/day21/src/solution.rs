use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut l = input.lines();
    let mut p1 = l.next().unwrap().trim()[28..].parse::<usize>().unwrap() - 1;
    let mut p2 = l.next().unwrap().trim()[28..].parse::<usize>().unwrap() - 1;
    let mut s1 = 0;
    let mut s2 = 0;
    let mut d = 1;
    let mut c = 0;
    loop {
        let mut r = d; d += 1; if d > 100 { d = 1; }
        r += d; d += 1; if d > 100 { d = 1; }
        r += d; d += 1; if d > 100 { d = 1; }
        p1 += r;
        p1 %= 10;
        s1 += p1+1;
        c += 3;
        if s1 >= 1000 || s2 >= 1000 { break; }

        if s1 >= 1000 || s2 >= 1000 { break; }
        let mut r = d; d += 1; if d > 100 { d = 1; }
        r += d; d += 1; if d > 100 { d = 1; }
        r += d; d += 1; if d > 100 { d = 1; }
        p2 += r;
        p2 %= 10;
        s2 += p2+1;
        c += 3;
        if s1 >= 1000 || s2 >= 1000 { break; }
    }
    s1.min(s2) * c
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut l = input.lines();
    let mut p1 = l.next().unwrap().trim()[28..].parse::<usize>().unwrap() - 1;
    let mut p2 = l.next().unwrap().trim()[28..].parse::<usize>().unwrap() - 1;
    let mut dp = [[[[0; 10]; 10]; 21]; 21];
    dp[0][0][p1][p2] = 1;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut turn = true;
    let mut x = 0;
    let mut y = 0;
    loop {
        let mut dp2 = [[[[0; 10]; 10]; 31]; 31];
        for (i, j, l, m, a, b, c) in iproduct!(x..21, y..21, 0..10, 0..10, 1..4, 1..4, 1..4) {
            if turn {
                let p = (l + a + b + c) % 10;
                dp2[i + p + 1][j][p][m] += dp[i][j][l][m];
            } else {
                let p = (m + a + b + c) % 10;
                dp2[i][j + p + 1][l][p] += dp[i][j][l][m];
            }
        }
        if turn {
            c1 += dp2[21..]
                .iter()
                .flat_map(|a| &a[y..21])
                .flatten()
                .flatten()
                .sum::<u64>();
        } else {
            c2 += dp2[x..21]
                .iter()
                .flat_map(|a| &a[21..])
                .flatten()
                .flatten()
                .sum::<u64>();
        }
        let mut z = true;
        for (i, j, l, m) in iproduct!(x..21, y..21, 0..10, 0..10) {
            if dp2[i][j][l][m] != 0 {
                z = false;
            }
            dp[i][j][l][m] = dp2[i][j][l][m];
        }
        if z {
            break;
        }
        if turn {
            x += 1;
        } else {
            y += 1;
        }
        turn = !turn;
    }
    c1.max(c2)
}
