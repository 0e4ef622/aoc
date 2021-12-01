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
    let v: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut c = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            c += 1;
        }
    }
    c
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let v: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let v = v.windows(3).map(|x| x.iter().sum::<i64>()).collect::<Vec<_>>();
    let mut c = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            c += 1;
        }
    }
    c
}
