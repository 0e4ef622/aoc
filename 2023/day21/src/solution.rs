use std::{collections::*, mem::swap};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

use Dir::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    let mut g2 = g.clone();
    for i in 0..64 {
        step(&mut g, &mut g2);
    }

    let mut ans = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            if g[y][x] == b'S' { ans+=1; }
        }
    }
    ans
}

fn step(g: &mut Grid<u8>, g2: &mut Grid<u8>) {
    for y in 0..g.height {
        for x in 0..g.width {
            if g[y][x] != b'S' { continue; }
            let p = P(x as i64, y as i64);
            for dir in [U,D,L,R] {
                let p2 = p + dir.p();
                if p2.0 >= 0 && p2.0 < g.width as i64 && p2.1 >= 0 && p2.1 < g.height as i64 {
                    if g[p2.1 as usize][p2.0 as usize] != b'#' {
                        g2[p2.1 as usize][p2.0 as usize] = b'S';
                    }
                }
            }
            g2[y][x] = b'.';
        }
    }
    swap(g, g2);
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut og = Grid::from_ascii(input);

    let mut counts = vec![];
    let mut starts = vec![];
    for y in 0..og.height {
        for x in 0..og.width {
            if og[y][x] == b'S' {
                starts.push(P(x as i64, y as i64));
            }
        }
    }

    let s = starts[0];
    og[s.au()] = b'.';
    starts = vec![
        P(0, 0), // top left 0
        P(s.0, 0), // top middle 1
        P(og.width as i64-1, 0), // top right 2
        P(0, s.1), // left middle 3
        s,           // middle 4
        P(og.width as i64-1, s.1), // right middle 5
        P(0, og.height as i64-1),  // bottom left 6
        P(s.0, og.height as i64-1), // bottom middle 7
        P(og.width as i64-1, og.height as i64-1), // bottom right 8
    ];

    for (ix, &st) in starts.iter().enumerate() {
        let mut g = og.clone();
        let mut g2 = g.clone();
        g[st.au()] = b'S';
        let mut c = vec![];
        for i in 0..400 {
            c.push(g.array.iter().filter(|x| **x == b'S').count());
            step(&mut g, &mut g2);
        }
        counts.push(c);
    }

    let steps = 26501365;
    // let steps = 64;
    let bound = steps/og.width as i64 + 4;
    let calculate = |x, y| {
        let i = if x<0 {
            if y > 0 {
                2
            } else if y == 0 {
                5
            } else {
                8
            }
        } else if x == 0 {
            if y > 0 {
                1
            } else if y == 0 {
                4
            } else {
                7
            }
        } else {
            if y > 0 {
                0
            } else if y == 0 {
                3
            } else {
                6
            }
        };
        let dp = starts[4] - (starts[i] + P(x, y)*og.width as i64);
        let dist = dp.0.abs() + dp.1.abs();
        let ci = steps - dist;
        if ci < 0 {
            0
        } else if ci < counts[i].len() as i64 {
            counts[i][ci as usize]
        } else {
            counts[i][(398 + ci%2) as usize]
        }
    };
    let mut ans = 0;
    let mut xb = 0;
    for y in -bound..bound {
        for x in -xb..-xb+8 {
            ans += (calculate(x, y));
        }
        let next = (-xb+8).max(xb-7);
        let diff = next - (-xb+8);
        let next = (-xb+8) + (diff/2*2);
        if next - (-xb+8) > 0 {
            let add = calculate((-xb+8), y) + calculate((-xb+8)+1, y);
            ans += add * (diff as usize/2);
        }

        for x in next..xb {
            ans += (calculate(x, y));
        }
        if y < 0 {
            xb += 1;
        } else {
            xb -= 1;
        }
    }
    ans
}
