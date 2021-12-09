use std::collections::*;
use rand::random;
use itertools::Itertools;
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cnt = 0;
    for line in input.lines() {
        let w = line.trim().split(" | ").cv();
        let _ten = w[0].split_whitespace().cv();
        let f = w[1].split_whitespace().cv();
        for v in f {
            if [2,4,3,7].contains(&v.len()) {
                cnt += 1;
            }
        }
    }
    cnt
}

const OK: [&'static [u8]; 10] = [b"abcefg", b"cf", b"acdeg", b"acdfg", b"bcdf", b"abdfg", b"abdefg", b"acf", b"abcdefg", b"abcdfg"];
pub fn good(p: &[u8], x: &[Vec<u8>]) -> bool {
    let k = OK.to_vec().chd(<[_]>::sort);
    let mut x = x.iter().map(|x| x.to_vec()).cv();
    x.iter_mut().flat_map(|v| v.iter_mut()).for_each(|c| *c = p[(*c-b'a') as usize]);
    x.iter_mut().for_each(|x| x.sort());
    x.sort();
    x == k
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let rv = OK.iter().copied().enumerate().map(|x| (x.1, x.0)).collect::<HashMap<_, _>>();
    let mut tot = 0;
    for line in input.lines() {
        let w = line.trim().split(" | ").cv();
        let ten = w[0].split_whitespace().map(|x| x.bytes().cv()).cv();
        let mut f = w[1].split_whitespace().map(|x| x.bytes().cv()).cv();

        for mut x in b"abcdefg".iter().copied().permutations(7) {
            if good(&x, &ten) {
                f.iter_mut().flat_map(|v| v.iter_mut()).for_each(|c| *c = x[(*c-b'a') as usize]);
                f.iter_mut().for_each(|v| v.sort());
                tot += rv[&f[0][..]]*1000 + rv[&f[1][..]]*100 + rv[&f[2][..]]*10 + rv[&f[3][..]];
            }
        }
    }
    tot
}
