use std::collections::*;
use rand::random;
use serde_scan::scan as s;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut tiles = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut c = 0;
        for ch in line.bytes() {
            match ch {
                b'e' if c == 0 => x += 1,
                b'e' if c == b's' => {x +=1; y -= 1; c = 0;}
                b'e' if c == b'n' => {y += 1; c = 0;}

                b'w' if c == 0 => x -= 1,
                b'w' if c == b's' => {y -= 1; c = 0;}
                b'w' if c == b'n' => {y += 1; x-=1; c = 0;}

                _ => c = ch,
            }
        }
        tiles.entry((x,y)).or_insert(false).app(|x| *x=!*x);
    }
    tiles.values().filter(|x|**x).count()
}

fn it(tiles: &mut HashMap<(i32,i32),bool>) {
    let mut adj = HashMap::new();
    for ((x, y), _) in tiles.iter().filter(|x| *x.1) {
        adj.entry((*x+1,*y)).or_insert(0).app(|x| *x+=1);
        adj.entry((*x-1,*y)).or_insert(0).app(|x| *x+=1);
        adj.entry((*x,*y+1)).or_insert(0).app(|x| *x+=1);
        adj.entry((*x,*y-1)).or_insert(0).app(|x| *x+=1);
        adj.entry((*x-1,*y+1)).or_insert(0).app(|x| *x+=1);
        adj.entry((*x+1,*y-1)).or_insert(0).app(|x| *x+=1);
    }

    let mut new = HashMap::new();
    for x in adj {
        if *tiles.get(&x.0).unwrap_or(&false) {
            if [1,2].contains(&x.1) {
                new.insert(x.0, true);
            }
        } else {
            if x.1 == 2 {
                new.insert(x.0, true);
            }
        }
    }
    *tiles = new;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut tiles = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut c = 0;
        for ch in line.bytes() {
            match ch {
                b'e' if c == 0 => x += 1,
                b'e' if c == b's' => {x +=1; y -= 1; c = 0;}
                b'e' if c == b'n' => {y += 1; c = 0;}

                b'w' if c == 0 => x -= 1,
                b'w' if c == b's' => {y -= 1; c = 0;}
                b'w' if c == b'n' => {y += 1; x-=1; c = 0;}

                _ => c = ch,
            }
        }
        tiles.entry((x,y)).or_insert(false).app(|x| *x=!*x);
    }

    for _ in 0..100 {
        it(&mut tiles);
    }
    tiles.values().filter(|x|**x).count()
}
