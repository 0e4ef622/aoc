use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut x = 1;
    let mut clock = 0;
    let mut wait = 0;
    let mut add = 0;
    let mut lines = input.lines();
    let mut sig = vec![];
    loop {
        clock += 1;
        if (clock % 40) == 20 {
            sig.push(x * clock);
        }
        if wait > 0 {
            wait -= 1;
            if wait == 0 {
                x += add;
            }
        } else {
            match lines.next() {
                Some(l) => {
                    let w = l.split_whitespace().cv();
                    match w[0] {
                        "noop" => (),
                        "addx" => {
                            wait = 1;
                            add = w[1].parse::<i64>().unwrap();
                        }
                        _ => (),
                    }
                }
                None => break,
            }
        }
    }
    sig.iter().sum::<i64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut x = 1;
    let mut clock = 0;
    let mut wait = 0;
    let mut add = 0;
    let mut lines = input.lines();
    let mut crt = vec![];
    loop {
        {
            let xc: i64 = clock % 40;
            if (x-xc).abs() <= 1 {
                crt.push('#');
            } else {
                crt.push(' ');
            }
        }
        clock += 1;
        if wait > 0 {
            wait -= 1;
            if wait == 0 {
                x += add;
            }
        } else {
            match lines.next() {
                Some(l) => {
                    let w = l.split_whitespace().cv();
                    match w[0] {
                        "noop" => (),
                        "addx" => {
                            wait = 1;
                            add = w[1].parse::<i64>().unwrap();
                        }
                        _ => (),
                    }
                }
                None => break,
            }
        }
    }
    println!("{}", crt.chunks(40).map(|c| c.iter().collect::<String>()).join("\n"));
    ""
}
