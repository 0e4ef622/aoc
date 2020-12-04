use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut r = 0;
    for line in input.split("\n\n") {
        let mut f = [false; 7];
        for x in line.split_whitespace() {
            match &x[..3] {
                "byr" => f[0] = true,
                "iyr" => f[1] = true,
                "eyr" => f[2] = true,
                "hgt" => f[3] = true,
                "hcl" => f[4] = true,
                "ecl" => f[5] = true,
                "pid" => f[6] = true,
                _ => (),
            }
        }
        if f.iter().all(|x| *x) {
            r += 1;
        }
    }
    r
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut r = 0;
    for line in input.split("\n\n") {
        let mut f = [false; 7];
        for x in line.split_whitespace() {
            if x.is_empty() { continue; }
            match &x[..3] {
                "byr" => f[0] = (1920..=2002).contains(&x[4..].parse::<usize>().unwrap()),
                "iyr" => f[1] = (2010..=2020).contains(&x[4..].parse::<usize>().unwrap()),
                "eyr" => f[2] = (2020..=2030).contains(&x[4..].parse::<usize>().unwrap()),
                "hgt" => {
                    f[3] = if &x[x.len()-2..] == "in" {
                        (59..=76).contains(&x[4..x.len()-2].parse::<usize>().unwrap())
                    } else if &x[x.len()-2..] == "cm" {
                        (150..=193).contains(&x[4..x.len()-2].parse::<usize>().unwrap())
                    } else { false };
                }
                "hcl" => f[4] = x[4..].len() == 7 && &x[4..5] == "#" && x[5..].chars().all(|x| (b'0'..=b'9').contains(&(x as u8)) || (b'a'..=b'f').contains(&(x as u8))),
                "ecl" => f[5] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&x[4..]),
                "pid" => f[6] = x[4..].len() == 9 && x[4..].bytes().all(|x| (b'0'..=b'9').contains(&x)),
                _ => (),
            }
        }
        if f.iter().all(|x| *x) {
            r += 1;
        }
    }
    r
}
