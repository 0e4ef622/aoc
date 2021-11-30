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

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut c = 0;
    for line in input.lines() {
        let ws = line.split_whitespace();
        let v: Vec<&str> = ws.clone().collect();
        let s: HashSet<&str> = ws.collect();
        if v.len() == s.len() {
            c += 1;
        }
    }
    c
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut c = 0;
    for line in input.lines() {
        let ws = line.split_whitespace();
        let v: Vec<Vec<u8>> = ws.map(|s| s.as_bytes().to_vec().chd(<[_]>::sort)).collect();
        let s: HashSet<Vec<u8>> = v.iter().cloned().collect();
        if v.len() == s.len() {
            c += 1;
        }
    }
    c
}
