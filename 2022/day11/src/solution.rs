use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

struct Monkey {
    items: Vec<i64>,
    op: Op,
    tes: i64,
    tru: usize,
    fal: usize,
    inspect: i64,
}

enum Op {
    TimesOld,
    PlusOld,
    Times(i64),
    Plus(i64),
}

fn gomonk(i: usize, monkeys: &mut [Monkey], p2: bool) {
    let mut items = std::mem::take(&mut monkeys[i].items);
    monkeys[i].inspect += items.len() as i64;

    for mut old in items.drain(..) {
        match monkeys[i].op {
            Op::TimesOld => old *= old,
            Op::PlusOld => old += old,
            Op::Times(a) => old *= a,
            Op::Plus(a) => old += a,
            _ => (),
        }
        if !p2 { old /= 3; }
        if old % monkeys[i].tes == 0 {
            let to = monkeys[i].tru;
            monkeys[to].items.push(old);
        } else {
            let to = monkeys[i].fal;
            monkeys[to].items.push(old);
        }
    }
    monkeys[i].items = items;
}

fn round(monkeys: &mut [Monkey], p2: bool) {
    for i in 0..monkeys.len() {
        gomonk(i, monkeys, p2);
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let sec = input.split("\n\n");
    let mut monkeys = vec![];
    for (i, se) in sec.enumerate() {
        let lines = se.lines().skip(1).cv();
        //   Starting items: 
        let items = lines[0][18..].split(", ").map(|x| x.parse::<i64>().unwrap()).cv();
        //   Operation: new = 
        let op = lines[1][19..].split_whitespace().skip(1).cv();
        //   Test: divisible by 
        let tes = lines[2][21..].trim().parse::<i64>().unwrap();
        //     If true: throw to monkey 
        let tru = lines[3][29..].parse().unwrap();
        let fal = lines[4][30..].parse().unwrap();
        let op = match op[..] {
            ["*", "old"] => Op::TimesOld,
            ["+", "old"] => Op::PlusOld,
            ["*", a] => Op::Times(a.parse().unwrap()),
            ["+", a] => Op::Plus(a.parse().unwrap()),
            _ => panic!(),
        };

        monkeys.push(Monkey {
            items, op, tes, tru, fal, inspect: 0
        });
    }

    for i in 0..20 {
        round(&mut monkeys, false);
    }

    monkeys.sort_by_key(|m| -m.inspect);

    monkeys[0].inspect * monkeys[1].inspect
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    gcd(b, a%b)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let sec = input.split("\n\n");
    let mut monkeys = vec![];
    let mut lcm = 1;
    for (i, se) in sec.enumerate() {
        let lines = se.lines().skip(1).cv();
        //   Starting items: 
        let items = lines[0][18..].split(", ").map(|x| x.parse::<i64>().unwrap()).cv();
        //   Operation: new = 
        let op = lines[1][19..].split_whitespace().skip(1).cv();
        //   Test: divisible by 
        let tes = lines[2][21..].trim().parse::<i64>().unwrap();
        //     If true: throw to monkey 
        let tru = lines[3][29..].parse().unwrap();
        let fal = lines[4][30..].parse().unwrap();

        let op = match op[..] {
            ["*", "old"] => Op::TimesOld,
            ["+", "old"] => Op::PlusOld,
            ["*", a] => Op::Times(a.parse().unwrap()),
            ["+", a] => Op::Plus(a.parse().unwrap()),
            _ => panic!(),
        };

        monkeys.push(Monkey {
            items, op, tes, tru, fal, inspect: 0
        });
        lcm = lcm * tes / gcd(lcm, tes);
    }

    for i in 0..10000 {
        round(&mut monkeys, true);
        for m in &mut monkeys {
            m.items.iter_mut().for_each(|i| *i %= lcm);
        }
    }

    monkeys.sort_by_key(|m| -m.inspect);

    monkeys[0].inspect * monkeys[1].inspect
}
