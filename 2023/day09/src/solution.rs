use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};

fn predict(a: &mut [i32]) -> i32 {
    if a.iter().all_equal() {
        return a[0];
    }
    let n = a.len();
    for i in 0..n-1 {
        a[i] = a[i+1] - a[i];
    }
    a[n-1] + predict(&mut a[..n-1])
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    let mut nums = vec![];
    for line in input.lines() {
        nums.extend(line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()));
        ans += predict(&mut nums);
        nums.clear();
    }
    ans
}

fn predict2(a: &mut [i32]) -> i32 {
    if a.iter().all_equal() {
        return a[0];
    }
    let n = a.len();
    let first = a[0];
    for i in 0..n-1 {
        a[i] = a[i+1] - a[i];
    }
    first - predict2(&mut a[..n-1])
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    let mut nums = vec![];
    for line in input.lines() {
        nums.extend(line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()));
        ans += predict2(&mut nums);
        nums.clear();
    }
    ans
}

// funny lookup table

const fn cpredict<const N: usize>(mut a: [i32; N], n: usize) -> i32 {
    if n == 1 {
        return a[0];
    }
    let mut i = 0;
    while i < n-1 {
        a[i] = a[i+1] - a[i];
        i += 1;
    }
    a[n-1] + cpredict(a, n-1)
}

const LUT: [i32; 21] = {
    let mut arr = [0; 21];
    let mut out = [0; 21];
    let mut i = 0;
    while i < 21 {
        arr[i] = 1;
        out[i] = cpredict(arr, 21);
        arr[i] = 0;
        i += 1;
    }
    out
};

pub fn parse(mut s: &[u8]) -> i32 {
    let neg = s[0] == b'-';
    if neg {
        s = &s[1..];
    }
    let mut r = 0;
    for c in s {
        r = r*10 + (c - b'0') as i32;
    }
    if neg {
        -r
    } else {
        r
    }
}

pub fn part1_funny(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        ans += line
            .split_ascii_whitespace()
            .map(|x| parse(x.as_bytes()))
            .zip(LUT)
            .map(|(a, b)| a.overflowing_mul(b).0)
            .fold(0i32, |a, b| a.overflowing_add(b).0);
    }
    ans
}
