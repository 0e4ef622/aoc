use std::collections::*;
use rand::random;
use serde_scan::scan as s;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut regs = HashMap::new();
    for line in input.lines() {
        let mut w = line.split_whitespace();
        let reg = w.next().unwrap();
        let op = w.next().unwrap();
        let val: i64 = w.next().unwrap().parse().unwrap();
        w.next();
        let cr = w.next().unwrap();
        let cop = w.next().unwrap();
        let cval: i64 = w.next().unwrap().parse().unwrap();

        let op = match &op[..1] {
            "i" => |x: &mut i64, y| *x += y,
            "d" => |x: &mut i64, y| *x -= y,
            _ => unreachable!(),
        };

        match cop {
            ">" if  *regs.entry(cr).or_insert(0) > cval => op(regs.entry(reg).or_insert(0), val),
            "<" if  *regs.entry(cr).or_insert(0) < cval => op(regs.entry(reg).or_insert(0), val),
            ">=" if *regs.entry(cr).or_insert(0) >= cval => op(regs.entry(reg).or_insert(0), val),
            "<=" if *regs.entry(cr).or_insert(0) <= cval => op(regs.entry(reg).or_insert(0), val),
            "==" if *regs.entry(cr).or_insert(0) == cval => op(regs.entry(reg).or_insert(0), val),
            "!=" if *regs.entry(cr).or_insert(0) != cval => op(regs.entry(reg).or_insert(0), val),
            _ => (),
        }
    }
    *regs.values().max().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut regs = HashMap::new();
    let mut max = 0;
    for line in input.lines() {
        let mut w = line.split_whitespace();
        let reg = w.next().unwrap();
        let op = w.next().unwrap();
        let val: i64 = w.next().unwrap().parse().unwrap();
        w.next();
        let cr = w.next().unwrap();
        let cop = w.next().unwrap();
        let cval: i64 = w.next().unwrap().parse().unwrap();

        let op = match &op[..1] {
            "i" => |x: &mut i64, y| *x += y,
            "d" => |x: &mut i64, y| *x -= y,
            _ => unreachable!(),
        };

        match cop {
            ">" if  *regs.entry(cr).or_insert(0) > cval => op(regs.entry(reg).or_insert(0), val),
            "<" if  *regs.entry(cr).or_insert(0) < cval => op(regs.entry(reg).or_insert(0), val),
            ">=" if *regs.entry(cr).or_insert(0) >= cval => op(regs.entry(reg).or_insert(0), val),
            "<=" if *regs.entry(cr).or_insert(0) <= cval => op(regs.entry(reg).or_insert(0), val),
            "==" if *regs.entry(cr).or_insert(0) == cval => op(regs.entry(reg).or_insert(0), val),
            "!=" if *regs.entry(cr).or_insert(0) != cval => op(regs.entry(reg).or_insert(0), val),
            _ => (),
        }
        max = max.max(*regs.values().max().unwrap())
    }
    max
}
