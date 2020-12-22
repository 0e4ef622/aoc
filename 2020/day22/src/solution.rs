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

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();
    let mut sections = input.split("\n\n");

    sections.next().unwrap().lines().skip(1).for_each(|x| x.parse::<usize>().unwrap().app(|y| p1.push_back(y)));
    sections.next().unwrap().lines().skip(1).for_each(|x| x.parse::<usize>().unwrap().app(|y| p2.push_back(y)));

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        p2.iter().rev().enumerate().map(|x| (x.0+1)*x.1).sum::<usize>()
    } else {
        p1.iter().rev().enumerate().map(|x| (x.0+1)*x.1).sum::<usize>()
    }
}

fn re(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let mut cfg = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let decks = (p1.clone(), p2.clone());
        if cfg.contains(&decks) {
            return true;
        }
        cfg.insert(decks);

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if c1 <= p1.len() && c2 <= p2.len() {
            re(&mut p1.iter().take(c1).copied().collect(), &mut p2.iter().take(c2).copied().collect())
        } else {
            c1 > c2
        };

        if winner {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    p2.is_empty()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();
    let mut sections = input.split("\n\n");

    sections.next().unwrap().lines().skip(1).for_each(|x| x.parse::<usize>().unwrap().app(|y| p1.push_back(y)));
    sections.next().unwrap().lines().skip(1).for_each(|x| x.parse::<usize>().unwrap().app(|y| p2.push_back(y)));

    re(&mut p1, &mut p2);

    if p1.is_empty() {
        p2.iter().rev().enumerate().map(|x| (x.0+1)*x.1).sum::<usize>()
    } else {
        p1.iter().rev().enumerate().map(|x| (x.0+1)*x.1).sum::<usize>()
    }
}
