use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn ins(s: &HashMap<[u8; 2], usize>, pi: &[Vec<&str>]) -> HashMap<[u8; 2], usize> {
    let mut r = s.clone();
    for v in pi {
        if let Some(&c) = s.get(v[0].as_bytes()) {
            let p1 = [v[0].as_bytes()[0], v[1].as_bytes()[0]];
            let p2 = [v[1].as_bytes()[0], v[0].as_bytes()[1]];
            *r.entry(p1).or_insert(0) += c;
            *r.entry(p2).or_insert(0) += c;
            *r.get_mut(v[0].as_bytes()).unwrap() -= c;
        }
    }
    r
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let otemplate = inlines.next().unwrap();
    let mut template = otemplate.as_bytes().windows(2).map(|s| s.try_into().unwrap()).counts();
    inlines.next();
    let pi = inlines.map(|l| l.split(" -> ").cv()).cv();

    for i in 0..10 {
        template = ins(&template, &pi);
    }

    let mut counts = HashMap::new();
    counts.insert(otemplate.as_bytes()[0], 1);
    for ([_, b], c) in template {
        *counts.entry(b).or_insert(0) += c;
    }
    let (mut mn, mut mx) = counts.iter().map(|x| (*x.0, *x.1)).minmax_by_key(|x| x.1).into_option().unwrap();
    mx.1 - mn.1
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let otemplate = inlines.next().unwrap();
    let mut template = otemplate.as_bytes().windows(2).map(|s| s.try_into().unwrap()).counts();
    inlines.next();
    let pi = inlines.map(|l| l.split(" -> ").cv()).cv();

    for i in 0..40 {
        template = ins(&template, &pi);
    }

    let mut counts = HashMap::new();
    counts.insert(otemplate.as_bytes()[0], 1);
    for ([_, b], c) in template {
        *counts.entry(b).or_insert(0) += c;
    }
    let (mut mn, mut mx) = counts.iter().map(|x| (*x.0, *x.1)).minmax_by_key(|x| x.1).into_option().unwrap();
    mx.1 - mn.1
}
