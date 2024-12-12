use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = Grid::from_ascii(input);
    let mut dsu = Dsu::new(g.width*g.height);
    let id = |p: P<i64>| p.1 as usize*g.width + p.0 as usize;
    let mut perimeter = Grid::new(vec![0; g.width*g.height], g.width);
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            let p = P(x as i64, y as i64);
            let mut per = 4;
            for d in Dir::iter() {
                if let Some(&cc) = g.get(p + d) {
                    if c == cc {
                        dsu.merge(id(p), id(p+d));
                        per -= 1;
                    }
                }
            }
            perimeter[p] = per;
        }
    }

    let mut meh = HashMap::<usize, usize>::new();
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            let p = P(x, y);
            *meh.entry(dsu.find(id(p.ai()))).or_default() += perimeter[p];
        }
    }

    meh.iter().map(|(&k, &v)| dsu.s[k]*v).sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let g = Grid::from_ascii(input);
    let mut dsu = Dsu::new((g.width+2)*(g.height+2));
    let id = |p: P<i64>| (p.1+1) as usize*(g.width+2) + (p.0+1) as usize;
    for (y, r) in g.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            let p = P(x as i64, y as i64);
            let mut per = 4;
            for d in Dir::iter() {
                if let Some(&cc) = g.get(p + d) {
                    if c == cc {
                        dsu.merge(id(p), id(p+d));
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            let c = id(P(x, y).ai());
            if dsu.find(c) != c { continue; }
            for y in 0..=g.height {
                for x in 0..=g.width {
                    let p = P(x, y).ai();
                    let a = [
                        dsu.find(id(p+Dir::U+Dir::L)) == c,
                        dsu.find(id(p+Dir::U)) == c,
                        dsu.find(id(p+Dir::L)) == c,
                        dsu.find(id(p)) == c,
                    ];
                    let cc = match a {
                        [false, false, false, false] => 0,
                        [true, false, false, false] => 1,
                        [false, true, false, false] => 1,
                        [false, false, true, false] => 1,
                        [false, false, false, true] => 1,
                        [true, true, false, false] => 0,
                        [true, false, true, false] => 0,
                        [true, false, false, true] => 2,
                        [false, true, true, false] => 2,
                        [false, true, false, true] => 0,
                        [false, false, true, true] => 0,
                        [true, true, true, false] => 1,
                        [true, true, false, true] => 1,
                        [true, false, true, true] => 1,
                        [false, true, true, true] => 1,
                        [true, true, true, true] => 0,
                    };
                    ans += cc*dsu.s[c];
                }
            }
        }
    }

    ans
}
