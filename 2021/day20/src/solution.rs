use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn pixnum(img: &[Vec<u8>], r: usize, c: usize) -> usize {
    assert!(r > 0);
    assert!(c > 0);
    assert!(r < img.len()-1);
    assert!(c < img[0].len()-1);

    let x1 = (img[r-1][c-1] == b'#') as usize;
    let x2 = (img[r-1][c] == b'#') as usize;
    let x3 = (img[r-1][c+1] == b'#') as usize;
    let x4 = (img[r][c-1] == b'#') as usize;
    let x5 = (img[r][c] == b'#') as usize;
    let x6 = (img[r][c+1] == b'#') as usize;
    let x7 = (img[r+1][c-1] == b'#') as usize;
    let x8 = (img[r+1][c] == b'#') as usize;
    let x9 = (img[r+1][c+1] == b'#') as usize;
    (x1 << 8) |
    (x2 << 7) |
    (x3 << 6) |
    (x4 << 5) |
    (x5 << 4) |
    (x6 << 3) |
    (x7 << 2) |
    (x8 << 1) |
    x9
}

fn step(mut img: Vec<Vec<u8>>, alg: &[u8], ex: bool) -> Vec<Vec<u8>> {
    let c = if ex { b'.' } else { b'#' };
    img.insert(0, vec![c; img[0].len()]);
    img.insert(0, vec![c; img[0].len()]);
    img.insert(0, vec![c; img[0].len()]);
    img.push(vec![c; img[0].len()]);
    img.push(vec![c; img[0].len()]);
    img.push(vec![c; img[0].len()]);
    for v in &mut img {
        v.insert(0, c);
        v.insert(0, c);
        v.insert(0, c);
        v.push(c);
        v.push(c);
        v.push(c);
    }
    let mut out = vec![];
    for i in 1..img.len()-1 {
        out.push(vec![]);
        for j in 1..img[0].len()-1 {
            out.last_mut().unwrap().push(alg[pixnum(&img, i, j)]);
        }
    }
    out
}

fn pr(img: &[Vec<u8>]) {
    for r in img {
        for c in r {
            eprint!("{}", *c as char);
        }
        eprintln!();
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let sects = input.split("\n\n").cv();
    let alg = sects[0].bytes().cv();
    let img = sects[1].lines().map(|l| l.as_bytes().to_vec()).cv();
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    img.iter().flatten().filter(|c| **c == b'#').count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let sects = input.split("\n\n").cv();
    let alg = sects[0].bytes().cv();
    let img = sects[1].lines().map(|l| l.as_bytes().to_vec()).cv();
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    let img = step(img, &alg, true);
    let img = step(img, &alg, false);
    img.iter().flatten().filter(|c| **c == b'#').count()
}
