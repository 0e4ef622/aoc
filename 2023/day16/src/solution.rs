use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    U,D,L,R
}
fn mv(x: i32, y: i32, d: Dir) -> (i32, i32) {
    match d {
        Dir::U => (x, y-1),
        Dir::D => (x, y+1),
        Dir::L => (x-1, y),
        Dir::R => (x+1, y),
    }
}
use Dir::*;

pub fn sim(input: &str, sx: i32, sy: i32, d: Dir, debug: bool) -> i32 {
    let g = input.lines().map(str::as_bytes).cv();
    let mut q = VecDeque::new();
    q.push_back(((sx, sy), d));
    let bck = |(x,y)| x>=0 && x<g[0].len() as i32 && y>=0 && y<g.len() as i32;
    let mut v = HashSet::new();
    while let Some(((x,y),d)) = q.pop_front() {
        if !v.insert((x,y,d)) { continue; }
        match g[y as usize][x as usize] {
            b'.' => if bck(mv(x,y,d)) { q.push_back((mv(x,y,d), d)); },
            b'/' => {
                let dd = match d {
                    Dir::U => R,
                    Dir::D => L,
                    Dir::L => D,
                    Dir::R => U,
                };
                if bck(mv(x,y,dd)) { q.push_back((mv(x,y,dd), dd)); }
            }
            b'\\' => {
                let dd = match d {
                    Dir::U => L,
                    Dir::D => R,
                    Dir::L => U,
                    Dir::R => D,
                };
                if bck(mv(x,y,dd)) { q.push_back((mv(x,y,dd), dd)); }
            }
            b'-' => if d == U || d == D {
                if bck(mv(x,y,L)) { q.push_back((mv(x,y,L), L)); }
                if bck(mv(x,y,R)) { q.push_back((mv(x,y,R), R)); }
            } else {
                    if bck(mv(x,y,d)) { q.push_back((mv(x,y,d), d)); }
            }
            b'|' => if d == L || d == R {
                if bck(mv(x,y,U)) { q.push_back((mv(x,y,U), U)); }
                if bck(mv(x,y,D)) { q.push_back((mv(x,y,D), D)); }
            } else {
                    if bck(mv(x,y,d)) { q.push_back((mv(x,y,d), d)); }
            }
            _ => unreachable!(),
        }
    }
    let mut ans = 0;
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            if v.contains(&(j as i32, i as i32, L))
            || v.contains(&(j as i32, i as i32, R))
            || v.contains(&(j as i32, i as i32, U))
            || v.contains(&(j as i32, i as i32, D))
            {
                ans += 1;
                if debug {
                    eprint!("#");
                }
            } else {
                if debug {
                eprint!("{}", g[i][j] as char);
                }
            }
        }
        if debug {
            eprintln!();
        }
    }
    ans
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    sim(input, 0, 0, R, false)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(str::as_bytes).cv();
    let w = g[0].len() as i32;
    let h = g.len() as i32;

    let mut ans = 0;
    for i in 0..w {
        ans = ans.max(sim(input, i, 0, D, false));
        ans = ans.max(sim(input, i, h-1, U, false));
    }
    for i in 0..h {
        ans = ans.max(sim(input, 0, i, R, false));
        ans = ans.max(sim(input, w-1, i, L, false));
    }
    ans
}
