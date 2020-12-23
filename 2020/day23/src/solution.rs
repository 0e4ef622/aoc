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

fn it(cups: &mut Vec<usize>) {
    let min = 1;
    let max = 9;
    let mut target = cups[0] - 1;
    if target < min { target = max; }
    while cups.iter().enumerate().find(|x| *x.1==target && x.0 > 3) == None {
        target -= 1;
        if target < min { target = max; }
    }
    let put = cups.iter().enumerate().find(|x| *x.1==target).unwrap().0;
    cups[1..put+1].rotate_left(3);
    cups.rotate_left(1);
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cups = input.trim().bytes().map(|x| (x-b'0') as usize).collect::<Vec<_>>();
    for x in 0..100 {
        if x%5==0 { println!(); }
        // println!("{:?}", cups);
        it(&mut cups);
        // i = (i+1)%cups.len();
    }
    println!("{:?}", cups);
    "idk"
}

fn it2(cups: &[usize], next: &mut [usize], n: usize) {
    let min = 1;
    let max = 1000000;
    let mut target = n - 1;
    let a = next[n];
    let b = next[a];
    let c = next[b];
    if target < min { target = max; }
    while [a, b, c].contains(&target) {
        target -= 1;
        if target < min { target = max; }
    }
    next[n] = next[c];
    next[c] = next[target];
    next[target] = a;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let cups = input.trim().bytes().map(|x| (x-b'0') as usize).chain(10..=1000000).collect::<Vec<_>>();
    let mut next = vec![0; 1000001];
    for i in 0..cups.len()-1 {
        next[cups[i]] = cups[i+1];
    }
    next[1000000] = cups[0];

    let mut i = cups[0];
    for x in 0..10000000 {
        // println!("{:?}", cups);
        it2(&cups, &mut next, i);
        i = next[i];
    }
    // println!("{:?}", cups);
    next[1] * next[next[1]]
}
