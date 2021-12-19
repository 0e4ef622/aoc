use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Clone, Debug)]
enum Item {
    V(u8),
    P(Box<(Item, Item)>),
}

fn explode<'a>(i: &'a mut Item, pn: &mut Option<&'a mut u8>, exp: &mut Option<u8>, depth: usize) -> bool {
    match i {
        Item::V(v) => {
            if let Some(exp) = exp { *v += *exp; return true; }
            else { *pn = Some(v) }
        }
        Item::P(box (Item::V(l), Item::V(r))) if depth >= 4 && exp.is_none() => {
            if let Some(pn) = pn { **pn += *l; }
            *exp = Some(*r);
            *i = Item::V(0);
        }
        Item::P(box (l, r)) => {
            if explode(l, pn, exp, depth+1) { return true; }
            if explode(r, pn, exp, depth+1) { return true; }
        }
    }
    false
}

fn split(i: &mut Item) -> bool {
    match i {
        Item::V(v) if *v >= 10 => {
            *i = Item::P((Item::V(*v/2), Item::V((*v+1)/2)).into());
            return true;
        }
        Item::V(_) => (),
        Item::P(box (l, r)) => {
            if split(l) { return true; }
            if split(r) { return true; }
        }
    }
    false
}

fn reduce(i: &mut Item) {
    loop {
        let mut exp = None;
        explode(i, &mut None, &mut exp, 0);
        if exp.is_some() { continue; }
        if split(i)      { continue; }
        break;
    }
}

fn fmt(i: &Item, o: &mut String) {
    use std::fmt::Write;
    match i {
        Item::V(v) => write!(o, "{}", v).unwrap(),
        Item::P(box (l, r)) => {
            *o += "[";
            fmt(l, o);
            *o += ",";
            fmt(r, o);
            *o += "]";
        }
    }
}

fn mag(i: &Item) -> usize {
    match i {
        Item::V(v) => *v as usize,
        Item::P(box (l, r)) => 3*mag(l) + 2*mag(r),
    }
}

fn pr(i: &Item) {
    let mut s = String::new();
    fmt(i, &mut s);
    eprintln!("{}", s);
}

fn parse(s: &mut &str) -> Item {
    if &s[..1] == "[" {
        *s = &s[1..];
        let l = parse(s);
        *s = &s[1..];
        let r = parse(s);
        *s = &s[1..];
        Item::P((l, r).into())
    } else {
        let v = s.as_bytes()[0] - b'0';
        *s = &s[1..];
        Item::V(v)
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let mut i = parse(&mut lines.next().unwrap());
    for line in lines {
        let j = parse(&mut {line});
        i = Item::P((i, j).into());
        reduce(&mut i);
    }
    mag(&i)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let items = input.lines().map(|mut x| parse(&mut x)).cv();
    let mut mx = 0;
    for (l, r) in iproduct!(&items, &items) {
        let mut i = Item::P((l.clone(), r.clone()).into());
        reduce(&mut i);
        mx = mx.max(mag(&i));
    }
    mx
}
