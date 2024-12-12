use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ncnt = HashMap::new();
    for n in input.split_whitespace() {
        *ncnt.entry(n.parse::<usize>().unwrap()).or_default() += 1;
    }

    for i in 0..25 {
        let mut n2 = HashMap::new();
        for &j in ncnt.keys() {
            if j == 0 {
                *n2.entry(1).or_default() += ncnt.get(&j).unwrap_or(&0);
                continue;
            }

            let ndig = j.ilog10()+1;
            if ndig%2 == 0 {
                let l = j / 10usize.pow(ndig/2);
                let r = j % 10usize.pow(ndig/2);
                *n2.entry(l).or_default() += ncnt.get(&j).unwrap_or(&0);
                *n2.entry(r).or_default() += ncnt.get(&j).unwrap_or(&0);
            } else {
                *n2.entry(j*2024).or_default() += ncnt.get(&j).unwrap_or(&0);
            }
        }
        ncnt = n2;
    }
    ncnt.values().sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ncnt = HashMap::new();
    for n in input.split_whitespace() {
        *ncnt.entry(n.parse::<usize>().unwrap()).or_default() += 1;
    }

    for i in 0..75 {
        let mut n2 = HashMap::new();
        for (&j, &v) in &ncnt {
            if v == 0 { continue; }
            if j == 0 {
                if let Some(&k) = ncnt.get(&j) {
                    *n2.entry(1).or_default() += v;
                }
                continue;
            }

            let ndig = j.ilog10()+1;
            if ndig%2 == 0 {
                let l = j / 10usize.pow(ndig/2);
                let r = j % 10usize.pow(ndig/2);
                *n2.entry(l).or_default() += v;
                *n2.entry(r).or_default() += v;
            } else {
                *n2.entry(j*2024).or_default() += v;
            }
        }
        ncnt = n2;
    }
    ncnt.values().sum::<usize>()
}
