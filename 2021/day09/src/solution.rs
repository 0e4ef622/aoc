use std::collections::*;
use rand::random;
use itertools::Itertools;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut m = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for c in line.trim().bytes().enumerate() {
            m.insert((i as i32, c.0 as i32), (c.1 - b'0') as i32);
        }
    }

    let mut cn = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            let i = i as i32;
            let j = j as i32;
            let l = m.get(&(i, j-1)).unwrap_or(&10);
            let r = m.get(&(i, j+1)).unwrap_or(&10);
            let u = m.get(&(i-1, j)).unwrap_or(&10);
            let d = m.get(&(i+1, j)).unwrap_or(&10);
            let c = (c - b'0') as i32;
            if &c < l && &c < r && &c < u && &c < d {
                cn+=1+c;
            }
        }
    }
    cn
}

struct Dsu {
    p: Vec<usize>,
    s: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }
        Dsu { p, s: vec![1; n] }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        }
        let v = self.find(self.p[i]);
        self.p[i] = v;
        return self.p[i];
    }

    fn merge(&mut self, i: usize, j: usize) {
        eprintln!("{}, {}", i, j);
        let i = self.find(i);
        let j = self.find(j);
        if i == j { return; }
        self.p[i] = j;
        self.s[j] += self.s[i];
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut m = HashMap::new();
    let mut x = 0;
    for (i, line) in input.lines().enumerate() {
        for c in line.trim().bytes().enumerate() {
            m.insert((i as i32, c.0 as i32), ((c.1 - b'0') as i32, x));
            x += 1;
        }
    }

    let mut ds = Dsu::new(x);
    x = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            let i = i as i32;
            let j = j as i32;
            let l = m.get(&(i, j-1)).unwrap_or(&(9, 0));
            let r = m.get(&(i, j+1)).unwrap_or(&(9, 0));
            let u = m.get(&(i-1, j)).unwrap_or(&(9, 0));
            let d = m.get(&(i+1, j)).unwrap_or(&(9, 0));
            let c = (c - b'0') as i32;
            if c == 9 { x += 1;continue; }
            if l.0 != 9 { ds.merge(x, l.1); }
            if r.0 != 9 { ds.merge(x, r.1); }
            if u.0 != 9 { ds.merge(x, u.1); }
            if d.0 != 9 { ds.merge(x, d.1); }
            x += 1;
        }
    }

    let mut v = vec![];
    for i in 0..x {
        if ds.p[i] == i { v.push(ds.s[i]); }
    }
    v.sort();
    v[v.len()-3..].iter().product::<usize>()
}
