use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn re(s: &str, v: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if v.len() == 0 { return (!s.contains('#')) as usize; }
    if s.len() < v[0] { return 0; }
    let key = (s.as_ptr() as usize, v.as_ptr() as usize);
    {
        if memo.contains_key(&key) {
            return memo[&key];
        }
    }
    let mut ans = 0;
    if s[..v[0]].chars().all(|x| ['?','#'].contains(&x)) && (s.len() == v[0] || s.as_bytes()[v[0]] != b'#') {
        if s.len() == v[0] {
            if v.len() == 1 {
                ans += 1;
            }
        } else {
            ans += re(&s[v[0]+1..], &v[1..], memo);
        }
    }
    if s.as_bytes()[0] != b'#' {
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
