use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sizes = HashMap::new();
    let mut el = vec![];
    for line in input.lines() {
        let c = &line[..3];
        sizes.insert(c, 1);
        for j in line[5..].split_ascii_whitespace() {
            sizes.insert(j, 1);
            el.push((c, j));
        }
    }
    loop {
        let mut el = el.clone();
        let mut sizes = sizes.clone();
        for _ in 0..sizes.len()-2 {
            let i = random::<usize>() % el.len();
            let (a, b) = el.swap_remove(i);
            // contract edge
            *sizes.get_mut(a).unwrap() += sizes[b];
            for e in &mut el {
                if e.0 == b {
                    e.0 = a;
                }
                if e.1 == b {
                    e.1 = a;
                }
            }
            el.retain(|x| x.0 != x.1);
        }
        if el.len() == 3 {
            return sizes[el[0].0]*sizes[el[0].1];
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    "Merry Christmas!"
}
