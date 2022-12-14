use std::{collections::*, cmp::Ordering, fmt::{Display, Debug}};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Eq, PartialEq, Clone)]
enum Value {
    Int(i64),
    List(Vec<Value>),
}

impl Ord for Value {

    fn cmp(&self, r: &Value) -> Ordering {
        use Value::{Int, List};
        match (self, r) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Int(a), List(b)) => [Value::Int(*a)][..].cmp(b),
            (List(a), Int(b)) => (**a).cmp(&[Value::Int(*b)][..]),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => Display::fmt(n, f),
            Value::List(l) => Debug::fmt(l, f),
        }
    }
}

fn parse(s: &mut &str) -> Value {
    if &s[..1] == "[" {
        let mut list = vec![];
        loop {
            *s = &s[1..];
            if &s[..1] == "]" {
                *s = &s[1..];
                break Value::List(list);
            }
            list.push(parse(s));
            if &s[..1] == "]" {
                *s = &s[1..];
                break Value::List(list);
            }
        }
    } else {
        let (a, b) = s.split_once([',', ']']).unwrap();
        *s = &s[a.len()..];
        Value::Int(a.parse().unwrap())
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let lines = pair.lines().cv();
        let first = lines[0];
        let second = lines[1];
        let first2 = parse(&mut {first});
        let second2 = parse(&mut {second});

        if first2 <= second2 {
            ans += i+1;
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let div1 = parse(&mut "[[2]]");
    let div2 = parse(&mut "[[6]]");
    let mut pkts = vec![div1.clone(), div2.clone()];
    for (i, pair) in input.split("\n\n").enumerate() {
        let lines = pair.lines().cv();
        let first = lines[0];
        let second = lines[1];
        let first2 = parse(&mut {first});
        let second2 = parse(&mut {second});

        pkts.push(first2);
        pkts.push(second2);
    }
    pkts.sort_unstable();
    let a = pkts.iter().position(|v| v == &div1).unwrap();
    let b = pkts.iter().position(|v| v == &div2).unwrap();
    (a+1)*(b+1)
}
