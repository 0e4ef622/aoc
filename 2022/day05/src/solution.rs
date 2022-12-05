use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (sec1, sec2) = input.split_once("\n\n").unwrap();
    let lines = sec1.lines().cv();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; (lines.last().unwrap().len() + 1) / 4];
    for &line in &lines[..lines.len()-1] {
        for i in 0..stacks.len() {
            let x = &line[i*4+1..i*4+2];
            if x != " " {
                stacks[i].push(x.chars().next().unwrap());
            }
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    for line in sec2.lines() {
        let w = line.split_whitespace().cv();
        let n = w[1].parse::<usize>().unwrap();
        let from = w[3].parse::<usize>().unwrap() - 1;
        let to = w[5].parse::<usize>().unwrap() - 1;
        for i in 0..n {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }
    stacks.iter().map(|v| *v.last().unwrap()).collect::<String>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (sec1, sec2) = input.split_once("\n\n").unwrap();
    let lines = sec1.lines().cv();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; (lines.last().unwrap().len() + 1) / 4];
    for &line in &lines[..lines.len()-1] {
        for i in 0..stacks.len() {
            let x = &line[i*4+1..i*4+2];
            if x != " " {
                stacks[i].push(x.chars().next().unwrap());
            }
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    for line in sec2.lines() {
        let w = line.split_whitespace().cv();
        let n = w[1].parse::<usize>().unwrap();
        let from = w[3].parse::<usize>().unwrap() - 1;
        let to = w[5].parse::<usize>().unwrap() - 1;
        for i in 0..n {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
        let lastn = stacks[to].len()-n;
        stacks[to][lastn..].reverse();
    }
    stacks.iter().map(|v| *v.last().unwrap()).collect::<String>()
}
