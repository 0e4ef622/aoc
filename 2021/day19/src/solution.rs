use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn rot(a: &[Vec<i64>], dir: &str) -> Vec<Vec<i64>> {
    match dir {
        "xy" => a.iter().cloned().map(|mut v| { (v[0], v[1]) = (v[1], -v[0]); v}).cv(),
        "yz" => a.iter().cloned().map(|mut v| { (v[1], v[2]) = (v[2], -v[1]); v}).cv(),
        "xz" => a.iter().cloned().map(|mut v| { (v[0], v[2]) = (v[2], -v[0]); v}).cv(),
        _ => todo!(),
    }
}

fn rots(a: &[Vec<i64>]) -> Vec<Vec<Vec<i64>>> {
    let mut r = vec![a.to_vec()];
    r.push(rot(&r[0], "xy"));
    r.push(rot(&r[1], "xy"));
    r.push(rot(&r[2], "xy"));

    r.push(rot(&r[0], "xz"));
    r.push(rot(&r[4], "xy"));
    r.push(rot(&r[5], "xy"));
    r.push(rot(&r[6], "xy"));

    r.push(rot(&r[1], "xz"));
    r.push(rot(&r[8], "xy"));
    r.push(rot(&r[9], "xy"));
    r.push(rot(&r[10], "xy"));

    r.push(rot(&r[2], "xz"));
    r.push(rot(&r[12], "xy"));
    r.push(rot(&r[13], "xy"));
    r.push(rot(&r[14], "xy"));

    r.push(rot(&r[3], "xz"));
    r.push(rot(&r[16], "xy"));
    r.push(rot(&r[17], "xy"));
    r.push(rot(&r[18], "xy"));

    r.push(rot(&r[4], "xz"));
    r.push(rot(&r[20], "xy"));
    r.push(rot(&r[21], "xy"));
    r.push(rot(&r[22], "xy"));
    assert!(r.len() == 24);
    r
}

fn merge(a: &[Vec<i64>], b: &[Vec<i64>]) -> Option<(Vec<Vec<i64>>, [i64; 3])> {
    let aset: BTreeSet<_> = a.iter().cloned().collect();
    for (aa, bb) in iproduct!(&*a, b) {
        let d = [aa[0] - bb[0],aa[1] - bb[1],aa[2] - bb[2]];
        let bset: BTreeSet<_> = b.iter().map(|c| vec![c[0]+d[0], c[1]+d[1], c[2]+d[2]]).collect();
        // eprintln!("{:?}", bset);
        if aset.intersection(&bset).count() >= 12 {
            return Some((aset.union(&bset).cloned().collect(), d));
        }
    }
    None
}

fn rotmerge(a: &[Vec<i64>], b: &[Vec<i64>]) -> Option<(Vec<Vec<i64>>, [i64; 3])> {
    for r in rots(b) {
        if let Some(v) = merge(a, &r) {
            return Some(v);
        }
    }
    None
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sc = vec![];
    for sect in input.split("\n\n") {
        sc.push(sect.lines().skip(1).map(|s| s.split(',').map(|v| v.parse::<i64>().unwrap()).cv()).cv());
    }
    let mut a = sc.swap_remove(0);
    'a: loop {
        eprintln!("{}, {}", a.len(), sc.len());
        for i in 0..sc.len() {
            if let Some(o) = rotmerge(&a, &sc[i]) {
                a = o.0;
                sc.swap_remove(i);
                continue 'a;
            }
        }
        break;
    }
    a.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sc = vec![];
    for sect in input.split("\n\n") {
        sc.push(sect.lines().skip(1).map(|s| s.split(',').map(|v| v.parse::<i64>().unwrap()).cv()).cv());
    }
    let mut a = sc.swap_remove(0);
    let mut s = vec![[0; 3]];
    'a: loop {
        eprintln!("{}, {}", a.len(), sc.len());
        for i in 0..sc.len() {
            if let Some(o) = rotmerge(&a, &sc[i]) {
                a = o.0;
                s.push(o.1);
                sc.swap_remove(i);
                continue 'a;
            }
        }
        break;
    }
    iproduct!(&s, &s).map(|(a, b)| (a[0]-b[0]).abs() + (a[1]-b[1]).abs() + (a[2]-b[2]).abs()).max().unwrap()
}
