use std::{collections::*, ops::{Index, IndexMut}};
use itertools::{iproduct, Itertools};
use rustc_hash::FxHashMap;
use utils::*;

struct Grid<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(array: Vec<T>, width: usize) -> Self {
        let height = array.len() / width;
        Self { array, width, height }
    }
    pub fn len(&self) -> usize {
        self.height
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        &self.array[index * self.width..][..self.width]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.array[index * self.width..][..self.width]
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g = input.lines().map(|x| x.as_bytes().to_vec()).cv();
    let mut ans = 0;
    for col in 0..g[0].len() {
        let mut s = 0;
        let mut c = 0;
        for row in 0..g.len() {
            if g[row][col] == b'O' {
                c += 1;
                g[row][col] = b'.';
            }
            if g[row][col] == b'#' || row == g.len()-1 {
                for i in s..s+c {
                    g[i][col] = b'O';
                    ans += g.len()-i;
                }
                s = row+1;
                c = 0;
            }
        }
    }
    ans
}

fn north(g: &mut Grid<u8>) {
    for col in 0..g[0].len() {
        let mut s = 0;
        let mut c = 0;
        for row in 0..g.len() {
            if g[row][col] == b'O' {
                c += 1;
                g[row][col] = b'.';
            }
            if g[row][col] == b'#' || row == g.len()-1 {
                for i in s..s+c {
                    g[i][col] = b'O';
                }
                s = row+1;
                c = 0;
            }
        }
    }
}

fn south(g: &mut Grid<u8>) {
    for col in 0..g[0].len() {
        let mut s = g.len();
        let mut c = 0;
        for row in (0..g.len()).rev() {
            if g[row][col] == b'O' {
                c += 1;
                g[row][col] = b'.';
            }
            if g[row][col] == b'#' || row == 0 {
                for i in s-c..s {
                    g[i][col] = b'O';
                }
                s = row;
                c = 0;
            }
        }
    }
}

fn west(g: &mut Grid<u8>) {
    for mut row in 0..g.len() {
        let mut s = 0;
        let mut c = 0;
        for col in 0..g[row].len() {
            if g[row][col] == b'O' {
                c += 1;
                g[row][col] = b'.';
            }
            if g[row][col] == b'#' || col == g[0].len()-1 {
                for i in s..s+c {
                    g[row][i] = b'O';
                }
                s = col+1;
                c = 0;
            }
        }
    }
}

fn east(g: &mut Grid<u8>) {
    for mut row in 0..g.len() {
        let mut s = g[row].len();
        let mut c = 0;
        for col in (0..g[row].len()).rev() {
            if g[row][col] == b'O' {
                c += 1;
                g[row][col] = b'.';
            }
            if g[row][col] == b'#' || col == 0 {
                for i in s-c..s {
                    g[row][i] = b'O';
                }
                s = col;
                c = 0;
            }
        }
    }
}

fn prn(g: &[Vec<u8>]) {
    for row in 0..g.len() {
        for col in 0..g[0].len() {
            eprint!("{}", g[row][col] as char);
        }
        eprintln!();
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let w = input.lines().next().unwrap().len();
    let g = input.lines().flat_map(|x| x.as_bytes()).copied().cv();
    let mut g = Grid::new(g, w);
    let mut ans = 0;
    let mut hashmap = FxHashMap::default();
    hashmap.insert(g.array.clone(), 0);
    let mut i = 1;
    while i <= 1000000000 {
        north(&mut g);
        west(&mut g);
        south(&mut g);
        east(&mut g);
        match hashmap.entry(g.array.clone()) {
            hash_map::Entry::Occupied(o) => {
                let m = *o.get()-i;
                i += (1000000000-i)/m*m;
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(i);
            }
        }
        i += 1;
    }
    let mut ans = 0;
    for row in 0..g.len() {
        for col in 0..g[0].len() {
            if g[row][col] == b'O' {
                ans += g.len()-row;
            }
        }
    }
    ans
}
