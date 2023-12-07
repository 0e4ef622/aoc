use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn of_a_kind(v: &[u8], n: usize) -> bool {
    let mut v = v.to_vec();
    v.sort();
    for i in 0..v.len()-n+1 {
        if v[i..i+n].iter().all_equal() {
            return true;
        }
    }
    false
}

pub fn two_pair(v: &[u8]) -> bool {
    let mut v = v.to_vec();
    v.sort();
    (v[0] == v[1] || v[1] == v[2]) && (v[2] == v[3] || v[3] == v[4])
}

pub fn full_house(v: &[u8]) -> bool {
    let mut v = v.to_vec();
    v.sort();
    v[0..3].iter().all_equal() && v[3..].iter().all_equal() ||
    v[0..2].iter().all_equal() && v[2..].iter().all_equal()
}

pub fn convert(v: &mut [u8]) {
    for x in v {
        *x = match x {
            b'2'..=b'9' => *x-b'2',
            b'T' => 8,
            b'J' => 9,
            b'Q' => 10,
            b'K' => 11,
            b'A' => 12,
            _ => unreachable!(),
        }
    }
}

pub fn kind(a: &[u8]) -> u8 {
    if of_a_kind(a, 5) {
        6
    } else if of_a_kind(a, 4) {
        5
    } else if full_house(a) {
        4
    } else if of_a_kind(a, 3) {
        3
    } else if two_pair(a) {
        2
    } else if of_a_kind(a, 2) { // one pair
        1
    } else {
        0
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut hands = vec![];
    for line in input.lines() {
        let words = line.split_ascii_whitespace().cv();
        let mut cards = words[0].as_bytes().to_vec();
        let bid = words[1].parse::<i64>().unwrap();
        convert(&mut cards);
        hands.push((cards, bid));
    }
    hands.sort_by_key(|c| (kind(&c.0), c.0.clone()));
    let mut ans = 0;
    for (i, h) in hands.iter().enumerate() {
        ans += (i as i64+1)*h.1;
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    p2::part2(input)
}

mod p2 {
    use std::collections::*;
    use rand::random;
    use itertools::{iproduct, Itertools};
    use util::*;
    pub fn of_a_kind(v: &[u8], n: usize) -> bool {
        let mut v = v.to_vec();
        v.sort();
        let jc = v.iter().take_while(|x| **x == 0).count();
        for i in 0..v.len()-n+1 {
            if v[i..i+n].iter().all_equal() {
                return true;
            }
        }
        jc != 0 && of_a_kind(&v[jc..], n-jc)
    }

    pub fn two_pair(v: &[u8]) -> bool {
        let mut v = v.to_vec();
        v.sort();
        (v[0] == v[1] || v[1] == v[2]) && (v[2] == v[3] || v[3] == v[4])
    }

    pub fn full_house(v: &[u8]) -> bool {
        let mut v = v.to_vec();
        v.sort();
        let last = *v.last().unwrap();
        for x in &mut v {
            if *x == 0 { *x = if last != 0 { last } else { 1 }; }
        }
        v.sort();
        v[0..3].iter().all_equal() && v[3..].iter().all_equal() ||
        v[0..2].iter().all_equal() && v[2..].iter().all_equal()
    }

    pub fn kind(a: &[u8]) -> u8 {
        if of_a_kind(a, 5) {
            6
        } else if of_a_kind(a, 4) {
            5
        } else if full_house(a) {
            4
        } else if of_a_kind(a, 3) {
            3
        } else if two_pair(a) {
            2
        } else if of_a_kind(a, 2) { // one pair
            1
        } else {
            0
        }
    }

    pub fn convert(v: &mut [u8]) {
        for x in v {
            *x = match x {
                b'2'..=b'9' => *x-b'2'+1,
                b'T' => 9,
                b'J' => 0,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => unreachable!(),
            }
        }
    }

    pub fn part2(input: &str) -> impl std::fmt::Display {
        let mut hands = vec![];
        for line in input.lines() {
            let words = line.split_ascii_whitespace().cv();
            let mut cards = words[0].as_bytes().to_vec();
            let bid = words[1].parse::<i64>().unwrap();
            convert(&mut cards);
            hands.push((cards, bid));
        }
        hands.sort_by_key(|c| (kind(&c.0), c.0.clone()));
        let mut ans = 0;
        for (i, h) in hands.iter().enumerate() {
            ans += (i as i64+1)*h.1;
        }
        ans
    }
}
