use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

fn print_grid(g: &Vec<Vec<u8>>) {
    for row in g {
        for col in row {
            if *col == 0 {
                print!(".");
            } else {
                print!("{col}");
            }
        }
        println!();
    }
}

fn apply_shape(row: usize, col: usize, g: &mut Vec<Vec<u8>>, shape: &[u8; 9]) -> bool {
    let w = g[0].len();
    let h = g.len();
    if !(0..h).contains(&(row+2)) || !(0..w).contains(&(col+2)) {
        return false;
    }
    for i in 0..3 {
        for j in 0..3 {
            if shape[i*3 + j] != 0 && g[row + i][col + j] != 0 {
                return false;
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if shape[i*3 + j] != 0 {
                g[row + i][col + j] = shape[i*3 + j];
            }
        }
    }
    true
}

fn unapply_shape(row: usize, col: usize, g: &mut Vec<Vec<u8>>, shape: &[u8; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            if shape[i*3 + j] != 0 {
                g[row + i][col + j] = 0;
            }
        }
    }
}

fn pack(row: usize, col: usize, g: &mut Vec<Vec<u8>>, cnts: &mut [u8], shapes: &[(u8, [u8; 9])]) -> bool {
    if cnts.iter().all(|&x| x == 0) {
        return true;
    }

    let w = g[0].len();
    let h = g.len();
    if row == h { return false; }

    let next_row;
    let next_col;
    if col == w-1 {
        next_row = row + 1;
        next_col = 0;
    } else {
        next_row = row;
        next_col = col + 1;
    }

    // try every shape
    for (idx, shape) in shapes {
        let idx = *idx as usize;
        if cnts[idx] > 0 {
            if apply_shape(row, col, g, shape) {
                // print_grid(g);
                // println!();
                cnts[idx] -= 1;
                if pack(next_row, next_col, g, cnts, shapes) {
                    return true;
                }
                cnts[idx] += 1;
                unapply_shape(row, col, g, shape);
            }
        }
    }
    pack(next_row, next_col, g, cnts, shapes)
}

fn rotate(s: [u8; 9]) -> [u8; 9] {
    [
        s[6], s[3], s[0],
        s[7], s[4], s[1],
        s[8], s[5], s[2],
    ]
}

fn flip(s: [u8; 9]) -> [u8; 9] {
    [
        s[2], s[1], s[0],
        s[5], s[4], s[3],
        s[8], s[7], s[6],
    ]
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let chunks = input.split("\n\n").cv();
    let mut shape_fills = vec![];
    let shapes = chunks[..chunks.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(idx, &s)| {
            let mut r = [0; 9];
            shape_fills.push(0);
            for (i, line) in s.lines().skip(1).enumerate().take(3) {
                for j in 0..3 {
                    if line.as_bytes()[j] == b'#' {
                        r[i*3+j] = (idx + 1) as u8;
                        *shape_fills.last_mut().unwrap() += 1;
                    }
                }
            }
            let r1 = rotate(r);
            let r2 = rotate(r1);
            let r3 = rotate(r2);
            [
                r, r1, r2, r3,
                flip(r),
                flip(r1),
                flip(r2),
                flip(r3),
            ].map(|sh| (idx as u8, sh))
        })
        .collect::<HashSet<_>>().into_iter().cv();

    let mut specs = chunks.last().unwrap().trim().lines().map(|spec| {
        let (size, shape_cnts) = spec.split_once(": ").unwrap();
        let (w, h) = size.split_once('x').unwrap();
        let w = w.parse::<usize>().unwrap();
        let h = h.parse::<usize>().unwrap();
        let shape_cnts = shape_cnts.split_ascii_whitespace().map(|c| c.parse::<u8>().unwrap()).cv();
        (w, h, shape_cnts)
    }).cv();

    let mut ans = 0;
    let specs_len = specs.len();
    for (i, (w, h, shape_cnts)) in specs.iter_mut().enumerate() {

        let minspace: usize = shape_cnts.iter().zip(&shape_fills).map(|(a, b)| *a as usize* *b as usize).sum();
        if minspace > *w * *h {
            continue;
        }
        let mut g = vec![vec![0; *w]; *h];
        if pack(0, 0, &mut g, shape_cnts, &shapes) {
            ans += 1;
        } else {
            eprintln!("bad");
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
