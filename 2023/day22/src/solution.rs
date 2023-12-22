use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn fall(bricks: &mut Vec<(Vec<i64>, Vec<i64>)>) -> usize {
    let mut height_map = [[0i64; 10]; 10];
    let mut changed = 0;
    for b in bricks {
        let h = *height_map[b.0[1] as usize..b.1[1] as usize+1].iter().flat_map(|x| &x[b.0[0] as usize..b.1[0] as usize+1]).max().unwrap();
        if b.0[2] != h {
            changed += 1;
            let bh = b.1[2] - b.0[2];
            b.0[2] = h;
            b.1[2] = h+bh;
        }
        for row in &mut height_map[b.0[1] as usize..b.1[1] as usize+1] {
            row[b.0[0] as usize..b.1[0] as usize+1].fill(b.1[2]+1);
        }
    }
    changed
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut bricks = input.lines().map(|l| {
        let ends = l.split('~').cv();
        let a = ends[0].split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        let b = ends[1].split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        assert!(a <= b);
        (a, b)
    }).cv();
    bricks.sort_unstable_by_key(|x| x.0[2]);
    fall(&mut bricks);

    let mut ans = 0;
    for i in 0..bricks.len() {
        let mut bc = bricks.clone();
        bc.remove(i);
        ans += (fall(&mut bc) == 0) as usize;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut bricks = input.lines().map(|l| {
        let ends = l.split('~').cv();
        let a = ends[0].split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        let b = ends[1].split(',').map(|x| x.parse::<i64>().unwrap()).cv();
        assert!(a <= b);
        (a, b)
    }).cv();
    bricks.sort_unstable_by_key(|x| x.0[2]);
    fall(&mut bricks);

    let mut ans = 0;
    for i in 0..bricks.len() {
        let mut bc = bricks.clone();
        bc.remove(i);
        ans += fall(&mut bc);
    }
    ans
}
