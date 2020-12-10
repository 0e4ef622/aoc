use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut n = input.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    n.sort_unstable();
    n.insert(0, 0);
    n.push(n.last().unwrap() + 3);
    n.windows(2).map(|x| x[1] - x[0]).filter(|&x| x == 1).count() *
    n.windows(2).map(|x| x[1] - x[0]).filter(|&x| x == 3).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut n = input.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    n.sort_unstable();

    n.insert(0, 0);
    n.push(n.last().unwrap() + 3);
    let mut dp = vec![0u128; n.len()];
    dp[0] = 1;
    for i in 1..n.len() {
        if i >= 3 && n[i] - n[i-3] <= 3 {
            dp[i] += dp[i-3];
        }
        if i >= 2 && n[i] - n[i-2] <= 3 {
            dp[i] += dp[i-2];
        }
        dp[i] += dp[i-1];
    }
    *dp.last().unwrap()
}
