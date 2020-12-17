use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sections = input.split("\n\n");
    let mut ranges = vec![];
    for line in sections.next().unwrap().lines() {
        let line = line.split(": ").nth(1).unwrap();
        let s = line.split(" or ");
        for x in s {
            let mut s = x.split("-");
            let n1 = s.next().unwrap().parse::<isize>().unwrap();
            let n2 = s.next().unwrap().parse::<isize>().unwrap();
            ranges.push(n1..=n2);
        }
    }
    sections.next();
    let mut invalid = 0isize;
    for line in sections.next().unwrap().lines() {
        if &line[0..1] == "n" { continue; }
        for n in line.split(",").map(|x| x.parse::<isize>().unwrap()) {
            for r in &ranges {
                if r.contains(&n) {
                    invalid -= n;
                    break;
                }
            }
            invalid += n;
        }
    }
    invalid
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sections = input.split("\n\n");
    let mut ranges = vec![];
    for line in sections.next().unwrap().lines() {
        let line = line.split(": ").nth(1).unwrap();
        let mut s = line.split(" or ");

        let mut ss = s.next().unwrap().split("-");
        let n1 = ss.next().unwrap().parse::<isize>().unwrap();
        let n2 = ss.next().unwrap().parse::<isize>().unwrap();

        let mut ss = s.next().unwrap().split("-");
        let n3 = ss.next().unwrap().parse::<isize>().unwrap();
        let n4 = ss.next().unwrap().parse::<isize>().unwrap();
        ranges.push((n1..=n2, n3..=n4));
    }
    let mine = sections.next().unwrap().lines().nth(1).unwrap();
    let mut tickets = vec![];
    for line in sections.next().unwrap().lines() {
        if &line[0..1] == "n" { continue; }
        let mut badd = false;
        for n in line.split(",").map(|x| x.parse::<isize>().unwrap()) {
            let mut bad = true;
            for r in &ranges {
                if r.1.contains(&n) || r.0.contains(&n) {
                    bad = false;
                    break;
                }
            }
            if bad {
                badd = true;
                break;
            }
        }

        if !badd { tickets.push(line.split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>()); }
    }
    println!("{}", tickets.len());

    let mut field_map = vec![(1u64<<ranges.len())-1; ranges.len()];

    for ticket in &tickets {
        for (i, &n) in ticket.iter().enumerate() {
            for (j, r) in ranges.iter().enumerate() {
                if !r.0.contains(&n) && !r.1.contains(&n) {
                    field_map[i] &= !(1<<j);
                }
            }
        }
    }

    let mut fm = vec![0; ranges.len()];
    for _ in 0..field_map.len() {
        for i in 0..field_map.len() {
            let n = field_map[i];
            if n.count_ones() == 1 {
                fm[find_one(n)] = i;
                field_map.iter_mut().for_each(|x| *x &= !(1<<find_one(n)));
                break;
            }
        }
    }
    fm[..6].iter().map(|x| mine.split(",").nth(*x).unwrap().parse::<isize>().unwrap()).product::<isize>()
}

fn find_one(mut n: u64) -> usize {
    let mut x = 0;
    while n > 0 {
        x += 1;
        n /= 2;
    }
    x-1
}
