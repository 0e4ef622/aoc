use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut file = vec![];
    for (i, c) in input.trim().bytes().enumerate() {
        let x = if i%2 == 0 {
            i as i64/2
        } else {
            -1
        };
        file.extend(std::iter::repeat_n(x, (c - b'0') as usize));
    }

    let mut i = 0;
    while i < file.len() {
        if file[i] == -1 {
            while file[file.len()-1] == -1 { file.pop(); }
            let x = file.pop().unwrap();
            file[i] = x;
        }
        i += 1;
    }

    let mut ans = 0;
    for (i, n) in file.iter().enumerate() {
        ans += i as i64*n;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut file = BTreeSet::new();
    let mut p = 0;
    for (i, c) in input.trim().bytes().enumerate() {
        let c = (c - b'0') as usize;
        if i%2 == 0 {
            let x = i as i64/2;
            file.insert((p, p+c, x));
        } else {
            file.insert((p, p+c, -1));
        }
        p += c;
    }

    let mut file2 = file.clone();
    for (s, e, n) in file2.into_iter().rev() {
        if n == -1 { continue; }
        for &(ss, ee, nn) in &file {
            if nn != -1 { continue; }
            if ss > s { break; }
            if ee-ss >= e-s {
                file.remove(&(s, e, n));
                file.remove(&(ss, ee, nn));
                if ss != ss+e-s {
                    file.insert((ss, ss + e-s, n));
                }
                if ee-ss != e-s {
                    file.insert((ss + e-s, ee, -1));
                }
                break;
            }
        }
    }
    let mut res = vec![-1; input.len()*10];
    for (s, e, n) in file {
        res[s..e].fill(n);
    }

    let mut ans = 0;
    for (i, &n) in res.iter().enumerate() {
        if n != -1 {
            ans += i as i64*n;
        }
    }
    assert!(ans != 6259669288370);
    assert!(ans != 8446022420670);
    ans
}

fn doprint(file: &BTreeSet<(usize, usize, i64)>) {
    let mut res = vec![];
    for &(s, e, n) in file {
        res.resize(e, -1);
        res[s..e].fill(n);
    }
    for x in res {
        if x == -1 {
            print!(".");
        } else {
            print!("{x}");
        }
    }
    println!();
}
