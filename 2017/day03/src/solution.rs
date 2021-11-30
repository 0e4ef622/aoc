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

const input: i64 = 312051;

pub fn part1(_input: &str) -> impl std::fmt::Display {
    let x = (input as f64).sqrt().ceil() as i64;
    ((x.pow(2) - input) % (x-1) - (x/2)).abs() + (x/2)
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    let mut map = HashMap::new();
    map.insert((0,0),1);
    let mut p = (1,0);
    let mut d = (1,0);
    loop {
        map.insert(p, 0);
        for k in [-1, 0, 1] {
            for l in [-1, 0, 1] {
                if k == 0 && l == 0 { continue }
                let v = *map.get(&(p.0+k, p.1+l)).unwrap_or(&0);
                *map.get_mut(&p).unwrap() += v;
            }
        }
        if dbg!(*map.get(&p).unwrap()) > input {
            return *map.get(&p).unwrap();
        }
        let b = (-d.1, d.0);
        let x = (p.0 + b.0, p.1 + b.1);
        if !map.contains_key(&x) { d = b; }
        p.0 += d.0;
        p.1 += d.1;
    }
    0
}
