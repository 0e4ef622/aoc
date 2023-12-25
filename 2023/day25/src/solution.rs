use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut name_to_id = HashMap::new();
    let mut get_id = |s| {
        let size = name_to_id.len();
        *name_to_id.entry(s).or_insert(size)
    };
    let mut el = vec![];
    for line in input.lines() {
        let c = get_id(&line[..3]);
        for j in line[5..].split_ascii_whitespace() {
            el.push((c, get_id(j)));
        }
    }
    loop {
        let mut el = el.clone();
        let mut nodes = Dsu::new(name_to_id.len());
        let mut cnt = 0;
        while cnt < name_to_id.len()-2 {
            let i = random::<usize>() % el.len();
            let (a, b) = el.swap_remove(i);
            // contract edge
            if nodes.merge(a, b) {
                cnt += 1;
            }
        }
        el.retain(|x| nodes.find(x.0) != nodes.find(x.1));
        if el.len() == 3 {
            let a = nodes.find(el[0].0);
            let b = nodes.find(el[0].1);
            return nodes.s[a]*nodes.s[b];
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    "Merry Christmas!"
}
