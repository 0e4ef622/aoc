use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn re(s: &str, v: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if v.len() == 0 { return (!s.contains('#')) as usize; }
    if s.len() < v[0] { return 0; }
    let key = (s.as_ptr() as usize, v.as_ptr() as usize);
    if memo.contains_key(&key) { return memo[&key]; }

    let (head, tail) = s.split_at(v[0]);
    let mut ans = 0;
    let chunk_has_dot = head.contains('.');
    let hash_after_chunk = tail.starts_with('#');
    let can_place_chunk = !chunk_has_dot && !hash_after_chunk;
    if can_place_chunk {
        if tail.is_empty() {
            ans += (v.len() == 1) as usize;
        } else {
            // match the first chunk to the head of the string
            // skip the first char of tail because it must be '.' to separate chunks
            ans += re(&tail[1..], &v[1..], memo);
        }
    }
    if &s[..1] != "#" {
        // don't match the first character to anything
        ans += re(&s[1..], v, memo);
    }
    memo.insert(key, ans);
    ans
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let parts = line.split_ascii_whitespace().cv();
        let conds = parts[1].split(',').map(|x| x.parse::<usize>().unwrap()).cv();
        ans += re(parts[0], &conds, &mut Default::default());
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut ans = 0;
    for line in input.lines() {
        let parts = line.split_ascii_whitespace().cv();
        let conds = parts[1].split(',').map(|x| x.parse::<usize>().unwrap()).cv();
        let s = std::iter::repeat(parts[0]).take(5).join("?");
        let conds = conds.repeat(5);
        ans += re(&s, &conds, &mut Default::default());
    }
    ans
}
