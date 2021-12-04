use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

fn has_col(b: &[[bool; 5]; 5]) -> bool {
    for i in 0..5 {
        let mut ok = true;
        for j in 0..5 {
            ok &= b[j][i];
        }
        if ok { return true; }
    }
    false
}
fn has_row(b: &[[bool; 5]; 5]) -> bool {
    for j in 0..5 {
        let mut ok = true;
        for i in 0..5 {
            ok &= b[j][i];
        }
        if ok { return true; }
    }
    false
}

fn win(b: &[[bool; 5]; 5]) -> bool {
    has_col(b) || has_row(b)
}

fn set(b: &mut Vec<Vec<&str>>, t: &mut [[bool; 5]; 5], v: &str) {
    for i in 0..5 {
        for j in 0..5 {
            if b[i][j] == v {
                t[i][j] = true;
            }
        }
    }
}

fn score(b: &Vec<Vec<&str>>, t: &[[bool; 5]; 5]) -> i64 {
    let mut un = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !t[i][j] {
                un += b[i][j].parse::<i64>().unwrap();
            }
        }
    }
    un
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
            if win(t) && !w { i = j; }
        }
    }
    score(&board[i], &t[i]) * n.parse::<i64>().unwrap()
}
