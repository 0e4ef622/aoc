use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let n: Vec<u64> = inlines.map(|x| x.parse().unwrap()).collect();

    for i in &n {
        for j in &n {
            if i+j == 2020 {
                return i*j;
            }
        }
    }
    return 0;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut inlines = input.lines();
    let n: Vec<u64> = inlines.map(|x| x.parse().unwrap()).collect();

    for i in &n {
        for j in &n {
            for k in &n {
                if i+j+k == 2020 {
                    return i*j*k;
                    // println!("{}", i*j);
                }
            }
        }
    }
    return 0;
}
