use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

// The input consists of the same block repeated 14 times, each block has 3 unique constants
// Each block performs the following, where z is a stack
//
//     read w
//     x = z.top() + c2
//     if c1 == 26 { z.pop() }
//     if x != w {
//         z.push(w + c3)
//     }

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut c1 = vec![];
    let mut c2 = vec![];
    let mut c3 = vec![];
    for sec in input.split("inp w").filter(|s| s.len()>1) {
        let lines = sec.trim().lines().cv();
        c1.push(lines[3][6..].parse::<u8>().unwrap());
        c2.push(lines[4][6..].parse::<i8>().unwrap());
        c3.push(lines[14][6..].parse::<i8>().unwrap());
    }
    assert_eq!(c1.len(), 14);
    assert_eq!(c2.len(), 14);
    assert_eq!(c3.len(), 14);

    let mut st = vec![];
    let mut ans = vec![];

    for i in 0..14 {
        if c1[i] == 1 {
            st.push((ans.len(), 9+c3[i]));
            ans.push(9);
        } else {
            assert!(c2[i] <= 0);
            let (j, v) = st.pop().unwrap();
            if v + c2[i] > 9 {
                ans[j] -= v + c2[i] - 9;
                ans.push(9);
            } else {
                ans.push(v + c2[i]);
            }
        }
    }
    ans.iter().map(|n| n.to_string()).collect::<String>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut c1 = vec![];
    let mut c2 = vec![];
    let mut c3 = vec![];
    for sec in input.split("inp w").filter(|s| s.len()>1) {
        let lines = sec.trim().lines().cv();
        c1.push(lines[3][6..].parse::<u8>().unwrap());
        c2.push(lines[4][6..].parse::<i8>().unwrap());
        c3.push(lines[14][6..].parse::<i8>().unwrap());
    }
    assert_eq!(c1.len(), 14);
    assert_eq!(c2.len(), 14);
    assert_eq!(c3.len(), 14);

    let mut st = vec![];
    let mut ans = vec![];

    for i in 0..14 {
        if c1[i] == 1 {
            st.push((ans.len(), 1+c3[i]));
            ans.push(1);
        } else {
            assert!(c2[i] <= 0);
            let (j, v) = st.pop().unwrap();
            if v + c2[i] <= 0 {
                ans[j] += -(v + c2[i]) + 1;
                ans.push(1);
            } else {
                ans.push(v + c2[i]);
            }
        }
    }
    ans.iter().map(|n| n.to_string()).collect::<String>()
}
