use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

fn evolve(mut a: i64, n: usize) -> i64 {
    for i in 0..n {
        a = prune(mix(a, a*64));
        a = prune(mix(a, a/32));
        a = prune(mix(a, a*2048));
    }
    a
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nums = input.lines().map(|s| s.parse::<i64>().unwrap());

    let mut ans = 0;
    for a in nums {
        ans += evolve(a, 2000);
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let nums = input.lines().map(|s| s.parse::<i64>().unwrap()).cv();

    let mut tot = HashMap::<_, i64>::new();
    for &(mut a) in &nums {
        let mut seq = vec![a];
        for i in 0..2000 {
            a = evolve(a, 1);
            seq.push(a);
        }
        let mut subtot = HashMap::<_, i64>::new();
        let mut diff = seq.windows(2).map(|a| a[1]%10 - a[0]%10).cv();
        for (i, w) in diff.windows(4).enumerate() {
            let w = <[_; 4]>::try_from(w).unwrap();
            subtot.entry(w).or_insert(seq[i+4] % 10);
        }
        for (k, v) in subtot.into_iter() {
            *tot.entry(k).or_default() += v;
        }
    }
    dbg!(tot.iter().max_by_key(|x| x.1).unwrap());
    let ans = *tot.values().max().unwrap();
    assert_ne!(ans, 1464);
    ans
}
