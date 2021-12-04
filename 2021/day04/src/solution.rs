use rand::random;
use serde_scan::scan as s;
use std::collections::*;
use util::*;

fn has_col(b: &[[bool; 5]; 5]) -> bool {
    (0..5).any(|i| (0..5).all(|j| b[j][i]))
}
fn has_row(b: &[[bool; 5]; 5]) -> bool {
    b.iter().any(|v| v.iter().all(|x| *x))
}

fn win(b: &[[bool; 5]; 5]) -> bool {
    has_col(b) || has_row(b)
}

fn set<'a>(b: &mut Vec<Vec<&'a str>>, t: &mut [[bool; 5]; 5], v: &'a str) {
    (0..5)
        .flat_map(|i| (0..5).map(move |j| (i, j)))
        .for_each(|(i, j)| t[i][j] |= b[i][j] == v);
}

fn score(b: &Vec<Vec<&str>>, t: &[[bool; 5]; 5]) -> i64 {
    (0..5)
        .flat_map(|i| (0..5).map(move |j| (i, j)))
        .map(|(i, j)| !t[i][j] as i64 * b[i][j].parse::<i64>().unwrap())
        .sum()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut secs = input.split("\n\n");
    let mut nums = secs.next().unwrap().split(",").cv();
    nums.reverse();
    let mut board = vec![];
    let mut t = vec![];
    for sec in secs {
        board.push(sec.lines().map(|l| l.split_whitespace().cv()).cv());
        t.push([[false; 5]; 5]);
    }
    let mut n = "";
    while t.iter().find(|b| win(b)).is_none() {
        n = nums.pop().unwrap();
        for (b, t) in board.iter_mut().zip(&mut t) {
            set(b, t, n);
        }
    }
    let (i, b) = t.iter().enumerate().find(|b| win(b.1)).unwrap();
    score(&board[i], b) * n.parse::<i64>().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut secs = input.split("\n\n");
    let mut nums = secs.next().unwrap().split(",").cv();
    nums.reverse();
    let mut board = vec![];
    let mut t = vec![];
    for sec in secs {
        board.push(sec.lines().map(|l| l.split_whitespace().cv()).cv());
        t.push([[false; 5]; 5]);
    }
    let mut n = "";
    let mut i = 0;
    while !t.iter().all(|b| win(b)) {
        n = nums.pop().unwrap();
        for (j, (b, t)) in board.iter_mut().zip(&mut t).enumerate() {
            let w = win(t);
            set(b, t, n);
            if win(t) && !w {
                i = j;
            }
        }
    }
    score(&board[i], &t[i]) * n.parse::<i64>().unwrap()
}
