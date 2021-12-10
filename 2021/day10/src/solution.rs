use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut bad = HashMap::new();
    let mut bk = HashMap::new();
    bad.insert(')', 3);
    bad.insert(']', 57);
    bad.insert('}', 1197);
    bad.insert('>', 25137);
    bk.insert('>', '<');
    bk.insert(')', '(');
    bk.insert('}', '{');
    bk.insert(']', '[');
    let mut score = 0;
    for line in input.lines() {
        let mut st = vec![];
        for c in line.trim().chars() {
            if "<({[".contains(c) {
                st.push(c);
            } else if &bk[&c] == st.last().unwrap() {
                st.pop();
            } else {
                score += bad[&c];
                break;
            }
        }
    }
    score
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut bad = HashMap::new();
    let mut bk = HashMap::new();
    bad.insert('(', 1);
    bad.insert('[', 2);
    bad.insert('{', 3);
    bad.insert('<', 4);
    bk.insert('>', '<');
    bk.insert(')', '(');
    bk.insert('}', '{');
    bk.insert(']', '[');

    let mut scores = vec![];
    'a: for line in input.lines() {
        let mut st = vec![];
        let mut s = 0usize;
        for c in line.trim().chars() {
            if "<({[".contains(c) {
                st.push(c);
            } else if &bk[&c] == st.last().unwrap() {
                st.pop();
            } else {
                continue 'a;
            }
        }
        for c in st.iter().rev() {
            s *= 5;
            s += bad[c];
        }
        scores.push(s);
    }
    scores.sort();
    scores[scores.len()/2]
}
