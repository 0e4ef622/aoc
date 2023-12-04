use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let a = line.split(": ").cv();
        let b = a[1].split(" | ").map(|x| x.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).cv()).cv();
        let win = BTreeSet::from_iter(&b[0]);
        let have = &b[1];
        let cnt = have.into_iter().filter(|x| win.contains(x)).count();
        if cnt != 0 {
            ans += 1 << (cnt-1);
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cards = vec![];
    for line in input.lines() {
        let a = line.split(": ").cv();
        let b = a[1].split(" | ").map(|x| x.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).cv()).cv();
        let win = BTreeSet::from_iter(b[0].clone());
        let have = b[1].clone();
        cards.push((win, have));
    }
    let mut card_cnt = vec![0; cards.len() + 1];
    let mut acc = 1;
    let mut total = 0;
    for i in 0..cards.len() {
        acc += card_cnt[i];
        total += acc;
        let win = &cards[i].0;
        let cnt = cards[i].1.iter().filter(|x| win.contains(x)).count();
        card_cnt[i+1] += acc;
        card_cnt[i+cnt+1] -= acc;
    }
    total
}
