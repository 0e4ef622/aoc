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
    let mut inlines = input.lines();
    let mut v: Vec<i64> = inlines.map(|x| x.parse().unwrap()).collect();
    let mut i = 0;
    let mut c = 0;
    loop {
        c += 1;
        v[i as usize] += 1;
        i += v[i as usize] - 1;
        if !(0..v.len()).contains(&(i as usize)) {
            return c;
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let mut v: Vec<i64> = inlines.map(|x| x.parse().unwrap()).collect();
    let mut i = 0;
    let mut c = 0;
    loop {
        c += 1;
        if v[i as usize] >= 3 {
            v[i as usize] -= 1;
            i += v[i as usize] + 1;
        } else {
            v[i as usize] += 1;
            i += v[i as usize] - 1;
        }
        if !(0..v.len()).contains(&(i as usize)) {
            return c;
        }
    }
}
