use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn fold(dots: &mut HashSet<(i64, i64)>, mut f: &str) {
    let mut s = f[11..].split('=');
    let axis = s.next().unwrap();
    let n = s.next().unwrap().parse::<i64>().unwrap();

    let mut ndots = HashSet::new();

    if axis == "x" {
        for &(mut dot) in &*dots {
            if dot.0 > n {
                dot.0 = n + (n - dot.0);
            }
            ndots.insert(dot);
        }
        *dots = ndots;
    } else {
        for &(mut dot) in &*dots {
            if dot.1 > n {
                dot.1 = n + (n - dot.1);
            }
            ndots.insert(dot);
        }
        *dots = ndots;
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut s = input.split("\n\n");
    let mut dots = s.next().unwrap().lines().map(|s| {
        let x = s.split(',').cv();
        (x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap())
    }).collect::<HashSet<_>>();

    let f = s.next().unwrap().lines().next().unwrap();
    fold(&mut dots, f);
    dots.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut s = input.split("\n\n");
    let mut dots = s.next().unwrap().lines().map(|s| {
        let x = s.split(',').cv();
        (x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap())
    }).collect::<HashSet<_>>();

    for f in s.next().unwrap().lines() {
        fold(&mut dots, f);
    }

    let mnx = dots.iter().min_by_key(|x| x.0).unwrap().0;
    let mxx = dots.iter().max_by_key(|x| x.0).unwrap().0;
    let mny = dots.iter().min_by_key(|x| x.1).unwrap().1;
    let mxy = dots.iter().max_by_key(|x| x.1).unwrap().1;
    for y in mny..=mxy {
        for x in mnx..=mxx {
            if dots.contains(&(x, y)) {
                print!("#");
            } else { print!(" "); }
        }
        println!();
    }
    dots.len()
}
