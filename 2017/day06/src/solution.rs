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
    let mut input = [14,0,15,12,11,11,3,5,1,6,8,4,9,1,8,4];
    // let mut input = [0,2,7,0];
    let mut s = HashSet::new();
    let mut c = 0;
    s.insert(input);
    loop {
        c += 1;
        let mx = input.iter().enumerate().max_by_key(|x| (x.1, -(x.0 as isize))).unwrap().0;
        let m = input[mx];
        input[mx] = 0;
        for i in 0..m {
            input[(i + mx + 1) % input.len()] += 1;
        }
        if !s.insert(input) {
            return c;
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut input = [14,0,15,12,11,11,3,5,1,6,8,4,9,1,8,4];
    // let mut input = [0,2,7,0];
    let mut s = HashMap::new();
    let mut c = 0;
    s.insert(input, 0);
    loop {
        c += 1;
        let mx = input.iter().enumerate().max_by_key(|x| (x.1, -(x.0 as isize))).unwrap().0;
        let m = input[mx];
        input[mx] = 0;
        for i in 0..m {
            input[(i + mx + 1) % input.len()] += 1;
        }
        if s.contains_key(&input) {
            return c - s[&input];
        }
        s.insert(input, c);
    }
}
