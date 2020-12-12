use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut dx = 1;
    let mut dy = 0;
    for line in input.lines() {
        let n = line[1..].parse::<i32>().unwrap();
        match line.as_bytes()[0] {
            b'F' => { x += n*dx; y += n*dy; }
            b'N' => { y += n; }
            b'S' => { y -= n; }
            b'E' => { x += n; }
            b'W' => { x -= n; }
            b'L' => for _ in 0..n/90 { let t = dx; dx = -dy; dy = t; }
            b'R' => for _ in 0..n/90 { let t = dx; dx = dy; dy = -t; }
            _ => (),
        }
    }
    println!("{} {}", x, y);
    x.abs()+y.abs()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut dx = 10;
    let mut dy = 1;
    for line in input.lines() {
        let n = line[1..].parse::<i32>().unwrap();
        match line.as_bytes()[0] {
            b'F' => { x += n*dx; y += n*dy; }
            b'N' => { dy += n; }
            b'S' => { dy -= n; }
            b'E' => { dx += n; }
            b'W' => { dx -= n; }
            b'L' => for _ in 0..n/90 { let t = dx; dx = -dy; dy = t; }
            b'R' => for _ in 0..n/90 { let t = dx; dx = dy; dy = -t; }
            _ => (),
        }
    }
    println!("{} {}", x, y);
    x.abs()+y.abs()
}
