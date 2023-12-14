use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = input.lines().map(|x| x.as_bytes().to_vec()).cv();
    let mut ans = 0;
    for col in 0..g[0].len() {
        for mut row in 0..g.len() {
            if g[row][col] != b'O' { continue; }
            while row > 0 && g[row-1][col] == b'.' {
                g[row][col] = b'.';
                g[row-1][col] = b'O';
                row -= 1;
            }
            ans += g.len()-row;
        }
    }
    ans
}

fn north(g: &mut Vec<Vec<u8>>) {
    for col in 0..g[0].len() {
        for mut row in 0..g.len() {
            if g[row][col] != b'O' { continue; }
            while row > 0 && g[row-1][col] == b'.' {
                g[row][col] = b'.';
                g[row-1][col] = b'O';
                row -= 1;
            }
        }
    }
}

fn south(g: &mut Vec<Vec<u8>>) {
    for col in 0..g[0].len() {
        for mut row in (0..g.len()).rev() {
            if g[row][col] != b'O' { continue; }
            while row < g.len()-1 && g[row+1][col] == b'.' {
                g[row][col] = b'.';
                g[row+1][col] = b'O';
                row += 1;
            }
        }
    }
}

fn west(g: &mut Vec<Vec<u8>>) {
    for mut row in 0..g.len() {
        for mut col in 0..g[0].len() {
            if g[row][col] != b'O' { continue; }
            while col > 0 && g[row][col-1] == b'.' {
                g[row][col] = b'.';
                g[row][col-1] = b'O';
                col -= 1;
            }
        }
    }
}

fn east(g: &mut Vec<Vec<u8>>) {
    for mut row in 0..g.len() {
        for mut col in (0..g[0].len()).rev() {
            if g[row][col] != b'O' { continue; }
            while col < g[0].len()-1 && g[row][col+1] == b'.' {
                g[row][col] = b'.';
                g[row][col+1] = b'O';
                col += 1;
            }
        }
    }
}

fn prn(g: &[Vec<u8>]) {
    for row in 0..g.len() {
        for col in 0..g[0].len() {
            eprint!("{}", g[row][col] as char);
        }
        eprintln!();
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = input.lines().map(|x| x.as_bytes().to_vec()).cv();
    let mut ans = 0;
    let mut hashmap = HashMap::new();
    hashmap.insert(g.clone(), 0);
    let mut i = 1;
    while i <= 1000000000 {
        north(&mut g);
        west(&mut g);
        south(&mut g);
        east(&mut g);
        if let Some(j) = hashmap.get(&g) {
            let m = j-i;
            i += (1000000000-i)/m*m;
        } else {
            hashmap.insert(g.clone(), i);
        }
        i += 1;
    }
    let mut ans = 0;
    for row in 0..g.len() {
        for col in 0..g[0].len() {
            if g[row][col] == b'O' {
                ans += g.len()-row;
            }
        }
    }
    // prn(&g);
    ans
}
