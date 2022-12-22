use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn decode_path(s: &str) -> Vec<String> {
    let mut r = vec![String::new()];
    for c in s.chars() {
        match c {
            '0'..='9' => r.last_mut().unwrap().push(c),
            _ => {
                r.push(c.into());
                r.push(String::new());
            }
        }
    }
    r
}

fn find_empty(map: &[Vec<u8>], mut x: i64, mut y: i64, dx: i64, dy: i64) -> (i64, i64) {
    let w = map[0].len() as i64;
    let h = map.len() as i64;
    loop {
        let xx = x + dx;
        let yy = y + dy;
        if !(0..w).contains(&xx)
            || !(0..h).contains(&yy)
            || map[yy as usize][xx as usize] == b' '
        {
            break (x, y);
        }
        (x, y) = (xx, yy);
    }
}

fn mv(map: &[Vec<u8>], x: i64, y: i64, dx: i64, dy: i64) -> (i64, i64) {
    let w = map[0].len() as i64;
    let h = map.len() as i64;
    let xx = x + dx;
    let yy = y + dy;

    let (xx, yy) = if !(0..w).contains(&xx)
        || !(0..h).contains(&yy)
        || map[yy as usize][xx as usize] == b' '
    {
        find_empty(map, x, y, -dx, -dy)
    } else {
        (xx, yy)
    };
    if map[yy as usize][xx as usize] == b'#' {
        (x, y)
    } else {
        (xx, yy)
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (map, path) = input.split_once("\n\n").unwrap();
    let w = map.lines().map(|l| l.len()).max().unwrap();
    let map = map.lines().map(|line| line.as_bytes().to_vec().ch(|v| v.resize(w, b' '))).cv();
    let path = decode_path(path.trim());

    let w = map[0].len();
    let h = map.len();

    let mut x = map[0].iter().position(|&b| b != b' ').unwrap() as i64;
    let mut y = 0 as i64;
    let (mut dx, mut dy) = (1, 0);
    for piece in &path {
        match (&**piece, piece.parse::<i64>()) {
            (_, Ok(n)) => for _ in 0..n {
                (x, y) = mv(&map, x, y, dx, dy);
            }
            ("L", _) => (dx, dy) = (dy, -dx),
            ("R", _) => (dx, dy) = (-dy, dx),
            _ => {
                unreachable!();
            }
        }
    }
    let facing = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000*(y+1) + 4*(x+1) + facing
}


//  12
//  3
// 45
// 6

fn get_side(x: i64, y: i64) -> usize {
    if (1*SIDE..2*SIDE).contains(&x) && (0*SIDE..1*SIDE).contains(&y) {
        1
    } else if (2*SIDE..3*SIDE).contains(&x) && (0*SIDE..1*SIDE).contains(&y) {
        2
    } else if (1*SIDE..2*SIDE).contains(&x) && (1*SIDE..2*SIDE).contains(&y) {
        3
    } else if (0*SIDE..1*SIDE).contains(&x) && (2*SIDE..3*SIDE).contains(&y) {
        4
    } else if (1*SIDE..2*SIDE).contains(&x) && (2*SIDE..3*SIDE).contains(&y) {
        5
    } else if (0*SIDE..1*SIDE).contains(&x) && (3*SIDE..4*SIDE).contains(&y) {
        6
    } else {
        dbg!(x, y);
        panic!("wtf");
    }
}

#[derive(Debug)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

use Facing::*;

fn facing(dx: i64, dy: i64) -> Facing {
    match (dx, dy) {
        (1, 0) => Right,
        (0, 1) => Down,
        (-1, 0) => Left,
        (0, -1) => Up,
        _ => unreachable!(),
    }
}

const SIDE: i64 = 50;


fn mv2(map: &[Vec<u8>], x: i64, y: i64, dx: i64, dy: i64) -> (i64, i64, i64, i64) {
    let w = map[0].len() as i64;
    let h = map.len() as i64;
    let xx = x + dx;
    let yy = y + dy;

    let (xx, yy, dxx, dyy) = if !(0..w).contains(&xx)
        || !(0..h).contains(&yy)
        || map[yy as usize][xx as usize] == b' '
    {
        // wrapping hell
        //  12
        //  3
        // 45
        // 6

        match (get_side(x, y), facing(dx, dy)) {
            (1, Up) => (0, x+2*SIDE, 1, 0),
            (1, Left) => (0, 2*SIDE + SIDE-y-1, 1, 0),
            (2, Down) => (2*SIDE-1, x-SIDE, -1, 0),
            (2, Right) => (2*SIDE-1, SIDE-y-1+2*SIDE, -1, 0),
            (2, Up) => (x-2*SIDE, 4*SIDE-1, 0, -1),
            (3, Left) => (y-SIDE, 2*SIDE, 0, 1),
            (3, Right) => (y+SIDE, SIDE-1, 0, -1),
            (4, Up) => (SIDE, SIDE+x, 1, 0),
            (4, Left) => (SIDE, SIDE-(y-2*SIDE)-1, 1, 0),
            (5, Down) => (SIDE-1, x+2*SIDE, -1, 0),
            (5, Right) => (3*SIDE-1, SIDE-(y-2*SIDE)-1, -1, 0),
            (6, Right) => (y-2*SIDE, 3*SIDE-1, 0, -1),
            (6, Down) => (x+2*SIDE, 0, 0, 1),
            (6, Left) => (y-2*SIDE, 0, 0, 1),
            _ => unreachable!(),
        }
    } else {
        (xx, yy, dx, dy)
    };
    if map[yy as usize][xx as usize] == b'#' {
        (x, y, dx, dy)
    } else {
        (xx, yy, dxx, dyy)
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (map, path) = input.split_once("\n\n").unwrap();
    let w = map.lines().map(|l| l.len()).max().unwrap();
    let map = map.lines().map(|line| line.as_bytes().to_vec().ch(|v| v.resize(w, b' '))).cv();
    let path = decode_path(path.trim());

    let w = map[0].len();
    let h = map.len();

    let mut x = map[0].iter().position(|&b| b != b' ').unwrap() as i64;
    let mut y = 0 as i64;
    let (mut dx, mut dy) = (1, 0);
    for piece in &path {
        match (&**piece, piece.parse::<i64>()) {
            (_, Ok(n)) => for _ in 0..n {
                (x, y, dx, dy) = mv2(&map, x, y, dx, dy);
            }
            ("L", _) => (dx, dy) = (dy, -dx),
            ("R", _) => (dx, dy) = (-dy, dx),
            _ => {
                dbg!(piece);
                unreachable!();
            }
        }
    }
    let facing = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    1000*(y+1) + 4*(x+1) + facing
}
