use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(|l|
        l.bytes().map(|c| c as i64 - '0' as i64).cv()
    ).cv();
    let w = g[0].len();
    let h = g.len();

    let mut set = HashSet::new();
    for i in 0..h {
        let mut mx = -1;
        for j in 0..w {
            if g[i][j] > mx {
                mx = g[i][j];
                set.insert((i,j));
            }
        }
        let mut mx = -1;
        for j in (0..w).rev() {
            if g[i][j] > mx {
                mx = g[i][j];
                set.insert((i,j));
            }
        }
    }

    for j in 0..w {
        let mut mx = -1;
        for i in 0..h {
            if g[i][j] > mx {
                mx = g[i][j];
                set.insert((i,j));
            }
        }
        let mut mx = -1;
        for i in (0..h).rev() {
            if g[i][j] > mx {
                mx = g[i][j];
                set.insert((i,j));
            }
        }
    }
    set.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(|l|
        l.bytes().map(|c| c as i64 - '0' as i64).cv()
    ).cv();
    let w = g[0].len();
    let h = g.len();

    let mut set: HashMap<(usize,usize), Vec<usize>> = HashMap::new();
    let mut anser = 0;
    for i in 0..h {
        for j in 0..w {
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            let mut d = 0;
            for k in (0..j).rev() {
                a += 1;
                if g[i][k] >= g[i][j] { break; }
            }
            for k in j+1..w {
                b += 1;
                if g[i][k] >= g[i][j] { break; }
            }
            for k in (0..i).rev() {
                c += 1;
                if g[k][j] >= g[i][j] { break; }
            }
            for k in i+1..h {
                d += 1;
                if g[k][j] >= g[i][j] { break; }
            }
            anser = anser.max(a*b*c*d);
        }
    }
    anser
}
