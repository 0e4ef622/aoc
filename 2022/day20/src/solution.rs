use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn mix(a: &mut [i64]) {
    let mut val = (0..a.len()).cv();
    let mut pos = (0..a.len()).cv();
    for i in 0..a.len() {
        // eprintln!("a = {:?}", (0..a.len()).map(|i| a[val[i]]).cv());
        // eprintln!("val = {val:?}");
        // eprintln!("pos = {pos:?}");
        let x = pos[i] as i64;
        let y = (x + a[i]) % (a.len()-1) as i64;
        let y = (y -1 + a.len() as i64) % (a.len()-1) as i64;
        let x = x as usize;
        let y = y as usize;
        // dbg!(x,y);
        // eprintln!();
        if x < y {
            for j in x+1..=y {
                pos[val[j]] -= 1;
            }
            pos[val[x]] = y;
            val[x..y+1].rotate_left(1);
        } else if x > y {
            for j in y..x {
                pos[val[j]] += 1;
            }
            pos[val[x]] = y;
            val[y..x+1].rotate_right(1);
        }
    }
    let r = (0..a.len()).map(|i| a[val[i]]).cv();
    a.copy_from_slice(&r);
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut a = input.lines().map(|x| x.parse::<i64>().unwrap()).cv();
    mix(&mut a);
    let v0 = a.iter().position(|&x| x == 0).unwrap();
    a[(v0+1000)%a.len()]
    + a[(v0+2000)%a.len()]
    + a[(v0+3000)%a.len()]
}

fn mix2(a: &mut [i64]) {
    let mut val = (0..a.len()).cv();
    let mut pos = (0..a.len()).cv();
    for _ in 0..10 {
        for i in 0..a.len() {
            // eprintln!("a = {:?}", (0..a.len()).map(|i| a[val[i]]).cv());
            // eprintln!("val = {val:?}");
            // eprintln!("pos = {pos:?}");
            let x = pos[i] as i64;
            let y = (x + a[i]) % (a.len()-1) as i64;
            let y = (y -1 + a.len() as i64) % (a.len()-1) as i64;
            let x = x as usize;
            let y = y as usize;
            // dbg!(x,y);
            // eprintln!();
            if x < y {
                for j in x+1..=y {
                    pos[val[j]] -= 1;
                }
                pos[val[x]] = y;
                val[x..y+1].rotate_left(1);
            } else if x > y {
                for j in y..x {
                    pos[val[j]] += 1;
                }
                pos[val[x]] = y;
                val[y..x+1].rotate_right(1);
            }
        }
    }
    let r = (0..a.len()).map(|i| a[val[i]]).cv();
    a.copy_from_slice(&r);
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut a = input.lines().map(|x| x.parse::<i64>().unwrap()*811589153).cv();
    mix2(&mut a);
    let v0 = a.iter().position(|&x| x == 0).unwrap();
    a[(v0+1000)%a.len()]
    + a[(v0+2000)%a.len()]
    + a[(v0+3000)%a.len()]
}
