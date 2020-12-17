use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cubes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            cubes.insert((x as isize, y as isize, 0), c == '#');
        }
    }

    for _ in 0..6 {
        it(&mut cubes);
    }
    cubes.values().filter(|x| **x).count()
}

fn it(cubes: &mut HashMap<(isize, isize, isize), bool>) {
    let mut adj_cnt = HashMap::<_,usize>::new();
    for c in &*cubes {
        let (cd, a) = c;
        if !a { continue; }
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 { continue; }
                    *adj_cnt.entry((x+cd.0, y+cd.1, z+cd.2)).or_default() += 1;
                }
            }
        }
    }

    let mut new = HashMap::new();

    for cnt in adj_cnt {
        let (cd, cnt) = cnt;
        if !cubes.get(&cd).unwrap_or(&false) {
            if cnt == 3 {
                new.insert(cd, true);
            } else {
                new.insert(cd, false);
            }
        } else if cnt == 2 || cnt == 3 {
            new.insert(cd, true);
        } else {
            new.insert(cd, false);
        }
    }
    *cubes = new;
}

fn it2(cubes: &mut HashMap<(isize, isize, isize, isize), bool>) {
    let mut adj_cnt = HashMap::<_,usize>::new();
    for c in &*cubes {
        let (cd, a) = c;
        if !a { continue; }
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 { continue; }
                    *adj_cnt.entry((x+cd.0, y+cd.1, z+cd.2, w + cd.3)).or_default() += 1;
                    }
                }
            }
        }
    }

    let mut new = HashMap::new();

    for cnt in adj_cnt {
        let (cd, cnt) = cnt;
        if !cubes.get(&cd).unwrap_or(&false) {
            if cnt == 3 {
                new.insert(cd, true);
            } else {
                new.insert(cd, false);
            }
        } else if cnt == 2 || cnt == 3 {
            new.insert(cd, true);
        } else {
            new.insert(cd, false);
        }
    }
    *cubes = new;
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cubes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            cubes.insert((x as isize, y as isize, 0isize,0isize), c == '#');
        }
    }

    for _ in 0..6 {
        it2(&mut cubes);
    }
    cubes.values().filter(|x| **x).count()
}
