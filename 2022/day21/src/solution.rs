use std::{collections::*, str::FromStr, fmt::{Display, Write}};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Clone, Debug)]
enum Sad {
    E {
        left: Box<Sad>,
        right: Box<Sad>,
        op: Op,
    },
    L(i64),
    Humn,
}

impl Sad {
    fn print(&self, s: &mut String) {
        match self {
            Sad::L(n) => { write!(s, "{n}"); },
            Sad::E { left, right, op } => {
                *s += "(";
                left.print(s);
                write!(s, "{op}");
                right.print(s);
                *s += ")";
            }
            Sad::Humn => { write!(s, "x"); },
        }
    }
}

impl Op {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => return Err(()),
        })
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let (name, etc) = line.split_once(": ").unwrap();
        let etc = etc.split_whitespace().cv();
        if etc.len() == 1 {
            monkeys.insert(name, Ok(etc[0].parse::<i64>().unwrap()));
        } else {
            monkeys.insert(name, Err((
                etc[0],
                Op::from_str(etc[1]).unwrap(),
                etc[2],
            )));
        }
    }

    while monkeys["root"].is_err() {
        for monkey in monkeys.keys().copied().cv() {
            match monkeys[monkey] {
                Ok(_) => (),
                Err((l, op, r)) => {
                    if let (Ok(l), Ok(r)) = (monkeys[l], monkeys[r]) {
                        *monkeys.get_mut(monkey).unwrap() = Ok(op.apply(l, r));
                    }
                }
            }
        }
    }
    monkeys["root"].unwrap()
}

fn i_want_die(mnk: &HashMap<&str, Result<i64, (&str, Op, &str)>>, s: &str, eq: i64) -> Option<i64> {
    if s == "humn" { return Some(eq); }
    let (l, op, r) = mnk[s].unwrap_err();
    match (mnk[l], op, mnk[r]) {
        (Ok(n), Op::Add, _) => i_want_die(mnk, r, eq-n),
        (_, Op::Add, Ok(n)) => i_want_die(mnk, l, eq-n),
        (Ok(n), Op::Sub, _) => i_want_die(mnk, r, n-eq),
        (_, Op::Sub, Ok(n)) => i_want_die(mnk, l, eq+n),
        (_, Op::Div, Ok(n)) => i_want_die(mnk, l, eq*n),
        (Ok(n), Op::Mul, _) => i_want_die(mnk, r, eq/n),
        (_, Op::Mul, Ok(n)) => i_want_die(mnk, l, eq/n),
        _ => None,
    }
}

fn make_sad(mnk: &HashMap<&str, Result<i64, (&str, Op, &str)>>, s: &str) -> Sad {
    if s == "humn" { return Sad::Humn; }
    match mnk[s] {
        Ok(n) => Sad::L(n),
        Err((l, op, r)) => {
            let left = Box::new(make_sad(mnk, l));
            let right = Box::new(make_sad(mnk, r));
            Sad::E { left, right, op }
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let (name, etc) = line.split_once(": ").unwrap();
        let etc = etc.split_whitespace().cv();
        if etc.len() == 1 {
            monkeys.insert(name, Ok(etc[0].parse::<i64>().unwrap()));
        } else {
            monkeys.insert(name, Err((
                etc[0],
                Op::from_str(etc[1]).unwrap(),
                etc[2],
            )));
        }
    }
    let (l, _, r) = monkeys["root"].unwrap_err();
    monkeys.insert("humn", Err(("", Op::Add, "")));
    while monkeys[l].is_err() && monkeys[r].is_err() {
        for monkey in monkeys.keys().copied().cv() {
            if monkey == "root" || monkey == "humn" { continue; }
            match monkeys[monkey] {
                Ok(_) => (),
                Err((l, op, r)) if l != "humn" && r != "humn" => {
                    if let (Ok(l), Ok(r)) = (monkeys[l], monkeys[r]) {
                        *monkeys.get_mut(monkey).unwrap() = Ok(op.apply(l, r));
                    }
                }
                _ => (),
            }
        }
    }
    // if let Ok(l) = monkeys[l] {
    //     if let Some(a) = i_want_die(&monkeys, r, l) {
    //         return a.to_string();
    //     }
    // } else if let Ok(r) = monkeys[r] {
    //     if let Some(a) = i_want_die(&monkeys, l, r) {
    //         return a.to_string();
    //     }
    // }
    let mut s = String::new();
    make_sad(&monkeys, l).print(&mut s);
    s += "=";
    make_sad(&monkeys, r).print(&mut s);
    String::from("https://www.wolframalpha.com/input?i2d=true&i=") + &s.replace("/", "%2F").replace("+", "%2B")
}
