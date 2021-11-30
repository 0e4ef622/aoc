use std::collections::*;
use rand::random;
use serde_scan::scan as s;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

trait Chain: Sized {
    fn ch<F, T>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> (),
    {
        f(&mut self);
        self
    }
}
impl<T> Chain for T {}

trait ChainDeref: Sized + std::ops::DerefMut {
    fn chd<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self::Target) -> (),
    {
        f(&mut *self);
        self
    }
}
impl<T: std::ops::DerefMut> ChainDeref for T {}

pub fn part1(input: &str) -> &str {
    let mut a = BTreeSet::new();
    let mut b = BTreeSet::new();
    for line in input.lines() {
        let mut w = line.splitn(4, " ");
        let name = w.next().unwrap();
        let we = w.next().unwrap().app(|s| &s[1..s.len()-1]).parse::<i64>().unwrap();
        if w.next().is_some() {
            for nam in w.next().unwrap().split(", ") {
                b.insert(nam);
            }
        }
        a.insert(name);
    }
    *(&a - &b).iter().next().unwrap()
}

fn fillw<'a>(g: &BTreeMap<&'a str, (i64, Vec<&'a str>)>, w: &mut BTreeMap<&'a str, i64>, i: &'a str) {
    let mut sum = g[i].0;
    for c in &g[i].1 {
        fillw(g, w, c);
        sum += w[c];
    }
    w.insert(i, sum);
}

fn test(g: &BTreeMap<&str, (i64, Vec<&str>)>, i: &str) -> i64 {
    let mut sum = g[i].0;
    let mut p = None;
    for c in &g[i].1 {
        let r = test(g, c);
        if r == -1 { return -1; }
        if p == None {
            p = Some(r);
        } else if p != Some(r) {
            return -1;
        }
        sum += r;
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut a = BTreeMap::new();
    let mut p = BTreeMap::new();
    for line in input.lines() {
        let mut w = line.splitn(4, " ");
        let name = w.next().unwrap();
        let we = w.next().unwrap().app(|s| &s[1..s.len()-1]).parse::<i64>().unwrap();
        let e = a.entry(name).or_insert((we, vec![]));
        if w.next().is_some() {
            for nam in w.next().unwrap().split(", ") {
                e.1.push(nam);
                p.insert(nam, name);
            }
        }
    }
    let parent = part1(input);
    let mut w = BTreeMap::new();
    fillw(&a, &mut w, parent);
    // for (i, v) in w {
    //     println!("{}: {}", i, v);
    // }
    for v in a.keys().copied().collect::<Vec<_>>() {
        if v == parent { continue; }
        let ow = a[v].0;
        let pa = p[v];
        let re = a[pa].clone();
        let va = if *re.1.first().unwrap() == v {
            w[re.1.last().unwrap()]
        } else {
            w[re.1.first().unwrap()]
        };
        a.get_mut(v).unwrap().0 += va - w[v];
        // println!("{} = {}", v, a[v].0);
        if test(&a, parent) != -1 {
            return a[v].0;
        }
        // println!();
        a.get_mut(v).unwrap().0 -= va - w[v];
    }
    -1
}
