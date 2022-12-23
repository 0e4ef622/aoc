use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn propose(map: &HashSet<(i64, i64)>, x: i64, y: i64, dir: [Dir; 4]) -> Option<(i64, i64)> {
    if map.contains(&(x, y-1)) || map.contains(&(x-1, y-1)) || map.contains(&(x+1, y-1))
    || map.contains(&(x, y+1)) || map.contains(&(x-1, y+1)) || map.contains(&(x+1, y+1))
    || map.contains(&(x+1, y+1)) || map.contains(&(x+1, y-1)) || map.contains(&(x+1, y))
    || map.contains(&(x-1, y+1)) || map.contains(&(x-1, y-1)) || map.contains(&(x-1, y)) {
    for d in dir {
        match d {
            Dir::North => {
                if map.contains(&(x, y-1)) || map.contains(&(x-1, y-1)) || map.contains(&(x+1, y-1)) {
                    continue;
                }
                return Some((x, y-1));
            }
            Dir::South => {
                if map.contains(&(x, y+1)) || map.contains(&(x-1, y+1)) || map.contains(&(x+1, y+1)) {
                    continue;
                }
                return Some((x, y+1));
            }
            Dir::East => {
                if map.contains(&(x+1, y+1)) || map.contains(&(x+1, y-1)) || map.contains(&(x+1, y)) {
                    continue;
                }
                return Some((x+1, y));
            }
            Dir::West => {
                if map.contains(&(x-1, y+1)) || map.contains(&(x-1, y-1)) || map.contains(&(x-1, y)) {
                    continue;
                }
                return Some((x-1, y));
            }
        }
    }
    }
    None
}

fn empty_area(map: &HashSet<(i64, i64)>) -> i64 {
    let minx = map.iter().map(|&(x, _)| x).min().unwrap();
    let maxx = map.iter().map(|&(x, _)| x).max().unwrap();
    let miny = map.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = map.iter().map(|&(_, y)| y).max().unwrap();
    (maxy-miny+1) * (maxx-minx+1) - map.len() as i64
}

fn print_map(map: &HashSet<(i64, i64)>) {
    let minx = map.iter().map(|&(x, _)| x).min().unwrap();
    let maxx = map.iter().map(|&(x, _)| x).max().unwrap();
    let miny = map.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = map.iter().map(|&(_, y)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if map.contains(&(x, y)) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];
    for _ in 0..10 {
        let mut moves = HashMap::new();
        for &(x, y) in &elves {
            let Some((xx, yy)) = propose(&elves, x, y, dirs) else { continue };
            let Some(_) = moves.insert((xx, yy), Some((x, y))) else { continue };
            moves.insert((xx, yy), None);
        }

        for m in moves {
            let (to, Some(from)) = m else { continue };
            elves.remove(&from);
            elves.insert(to);
        }
        dirs.rotate_left(1);
    }
    empty_area(&elves)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];
    let mut i = 1;
    loop {
        let mut moves = HashMap::new();
        for &(x, y) in &elves {
            let Some((xx, yy)) = propose(&elves, x, y, dirs) else { continue };
            let Some(_) = moves.insert((xx, yy), Some((x, y))) else { continue };
            moves.insert((xx, yy), None);
        }
        if moves.is_empty() { break; }

        for m in moves {
            let (to, Some(from)) = m else { continue };
            elves.remove(&from);
            elves.insert(to);
        }
        dirs.rotate_left(1);
        i += 1;
    }
    i
}
