use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn step(o: &mut [Vec<u8>]) {
    o.iter_mut().flatten().for_each(|x| *x += 1);
}

fn flash(o: &mut [Vec<u8>]) -> usize {
    let mut z = o.iter().enumerate().flat_map(|v| v.1.iter().copied().enumerate().map(move |u| (v.0 as isize, u.0 as isize, u.1))).filter(|x| x.2 > 9).cv();
    let mut f = z.iter().copied().map(|x| (x.0, x.1)).collect::<HashSet<_>>();
    while let Some((i, j, _)) = z.pop() {
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                let yy = i + y;
                let xx = j + x;
                if o.get(yy as usize).is_none() || o[yy as usize].get(xx as usize).is_none() { continue; }
                o[yy as usize][xx as usize] += 1;
                if o[yy as usize][xx as usize] > 9 && !f.contains(&(yy, xx)) {
                    f.insert((yy, xx));
                    z.push((yy, xx, 0))
                }
            }
        }
    }
    for &(i, j) in &f {
        o[i as usize][j as usize] = 0;
    }
    f.len()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut v = input.lines().map(|s| s.bytes().map(|x| x-b'0').cv()).cv();

    let mut tot = 0;
    for i in 0..100 {
        step(&mut v);
        tot += flash(&mut v);
    }

    tot
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut v = input.lines().map(|s| s.bytes().map(|x| x-b'0').cv()).cv();

    let mut c = 1;
    loop {
        step(&mut v);
        if flash(&mut v) == v.len()*v[0].len() {
            return c;
        }
        c += 1;
    }
}
