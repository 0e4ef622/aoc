use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cards: Vec<u32> = (0..10007).collect();

    for line in input.lines() {
        if &line[0..1] == "c" {
            let n = line[4..].parse().unwrap();
            cut(&mut cards, n);
        } else if &line[5..6] == "w" {
            let n = line[20..].parse().unwrap();
            increment(&mut cards, n);
        } else {
            new_stack(&mut cards);
        }
    }
    cards.iter().enumerate().find(|(_, c)| c == &&2019).unwrap().0
}

fn new_stack(cards: &mut Vec<u32>) {
    cards.reverse()
}

fn cut(cards: &mut Vec<u32>, n: isize) {
    let len = cards.len();
    if n > 0 {
        cards.rotate_left(n as usize % len);
    } else {
        cards.rotate_right((-n) as usize % len);
    }
}

fn increment(cards: &mut Vec<u32>, n: usize) {
    let mut dealt = vec![false; cards.len()];
    let mut new = cards.clone();
    let mut rem = cards.len();
    let mut i = 0;
    while rem > 0 {
        if !dealt[i] {
            dealt[i] = true;
            new[i] = cards[cards.len()-rem];
            rem -= 1;
        }
        i = (i + n) % cards.len();
    }
    *cards = new;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let cards = 119315717514047i128;
    let mut ins = vec![];
    for line in input.lines() {
        if &line[0..1] == "c" {
            let mut n = line[4..].parse::<i128>().unwrap() % cards;
            if n < 0 { n += cards; }
            ins.push((0, n));
        } else if &line[5..6] == "w" {
            let n: i128 = line[20..].parse().unwrap();
            ins.push((1, n));
        } else {
            ins.push((2, 0));
        }
    }
    let mut m = 1;
    let mut c = 0;
    for inst in ins.iter().rev() {
        if inst.0 == 0 {
            let mut n = cards - inst.1;
            c += cards - n;
            c %= cards;

        } else if inst.0 == 1 {
            let n = inst.1;
            let x = modpow(n, cards-2, cards);
            m *= x;
            c *= x;
            m %= cards;
        } else {
            m = cards - m;
            c = cards - c;
            c += cards-1;
            c %= cards;
            m %= cards;
        }
    }
    let m1 = modpow(m-1, cards-2, cards);
    let r = (m - 1) * 2020 % cards;
    let r = (r + c) % cards;
    let r = r * modpow(m, 101741582076661, cards) % cards;
    let r = (r - c) % cards;
    (r * m1) % cards
}

fn modpow(mut b: i128, mut p: i128, m: i128) -> i128 {
    let mut a = 1;
    while p > 0 {
        if p % 2 == 0 {
            b *= b;
            b %= m;
            p /= 2;
        } else {
            a *= b;
            a %= m;
            p -= 1;
        }
    }
    a
}
