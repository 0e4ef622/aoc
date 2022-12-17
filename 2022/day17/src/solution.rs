use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

// positive y is up

const ROCKS: [&'static [(i64, i64)]; 5] = [
    &[(0,0), (1,0), (2,0), (3,0)],
    &[(1,0), (0,1), (1,1), (2,1), (1,2)],
    &[(2,2),(2,1),(0,0),(1,0),(2,0)],
    &[(0,0), (0,1), (0,2), (0,3)],
    &[(0,0), (1,0), (0,1), (1,1)],
];

fn check_collision(rock: usize, x: i64, y: i64, grid: &mut HashMap<(i64, i64), char>) -> bool {
    for &(dx, dy) in ROCKS[rock] {
        if grid.contains_key(&(x+dx, y+dy)) || x+dx >= 7 || x+dx < 0 {
            return true;
        }
    }
    false
}

/// Returns new height.
fn fall(rock: usize, heights: &mut [i64; 7], grid: &mut HashMap<(i64, i64), char>, ins: &[char], ins_i: &mut usize) {
    let mut x = 2;
    let mut y = *heights.iter().max().unwrap() + 4;
    loop {
        let nx = if ins[*ins_i] == '<' {
            x-1
        } else {
            x+1
        };
        *ins_i = (*ins_i + 1) % ins.len();
        if !check_collision(rock, nx, y, grid) {
            x = nx;
        }
        if check_collision(rock, x, y-1, grid) {
            break;
        }
        y = y-1;
    }
    for &(dx, dy) in ROCKS[rock] {
        grid.insert((x+dx, y+dy), '#');
        heights[(x+dx) as usize] = heights[(x+dx) as usize].max(y+dy);
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {

    let mut grid = HashMap::new();
    for i in 0..7 {
        grid.insert((i,0), '-');
    }

    let mut ins = input.trim().chars().cv();

    let mut ins_i = 0;
    let mut heights = [0; 7];
    for i in 0..2022 {
        fall(i%5, &mut heights, &mut grid, &ins, &mut ins_i);
        let h = heights.into_iter().max().unwrap();
    }

    // for y in (0..100).rev() {
    //     for x in 0..7 {
    //         eprint!("{}", grid.get(&(x, y)).unwrap_or(&' '));
    //     }
    //     eprintln!();
    // }

    heights.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {

    let mut grid = HashMap::new();
    for i in 0..7 {
        grid.insert((i,0), '-');
    }

    let mut ins = input.trim().chars().cv();

    let mut seen = HashMap::new();
    let mut heights = vec![0];
    let mut ins_i = 0;
    let mut h = [0; 7];
    let mut i = 0;
    let (cycle_base, cycle_len) = loop {
        let mh = h.into_iter().max().unwrap();
        let hh = [mh-h[0], mh-h[1], mh-h[2], mh-h[3], mh-h[4], mh-h[5], mh-h[6]];
        let pv = seen.insert((i%5, ins_i, hh), i);
        if let Some(pv) = pv {
            break (pv, i-pv);
        }
        fall(i%5, &mut h, &mut grid, &ins, &mut ins_i);
        heights.push(h.into_iter().max().unwrap());
        i += 1;
    };

    let mut r = 1000000000000;
    let mut ans = 0;
    r -= cycle_base;
    ans += heights[cycle_base];
    ans += (heights[cycle_base+cycle_len] - heights[cycle_base]) * (r / cycle_len) as i64;
    r %= cycle_len;
    ans += heights[cycle_base+r] - heights[cycle_base];

    ans
}
