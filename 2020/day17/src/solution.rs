use std::collections::*;
use rand::random;
use serde_scan::scan as s;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut cubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cubes.insert((x as isize, y as isize, 0));
            }
        }
    }

    for _ in 0..6 {
        it(&mut cubes);
    }
    cubes.len()
}

fn it(cubes: &mut HashSet<(isize, isize, isize)>) {
    let mut adj_cnt = HashMap::<_,usize>::new();
    for cd in &*cubes {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 { continue; }
                    *adj_cnt.entry((x+cd.0, y+cd.1, z+cd.2)).or_default() += 1;
                }
            }
        }
    }

    let mut new = Vec::new();

    for cnt in adj_cnt {
        let (cd, cnt) = cnt;
        if !cubes.contains(&cd) {
            if cnt == 3 {
                new.push(cd);
            }
        } else if cnt == 2 || cnt == 3 {
            new.push(cd);
        }
    }
    cubes.clear();
    cubes.extend(new);
}

fn it2(cubes: &mut HashSet<(isize, isize, isize, isize)>) {
    let mut adj_cnt = HashMap::<_,usize>::new();
    for cd in &*cubes {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 { continue; }
                        *adj_cnt.entry((x+cd.0, y+cd.1, z+cd.2, w+cd.3)).or_default() += 1;
                    }
                }
            }
        }
    }

    let mut new = Vec::new();

    for cnt in adj_cnt {
        let (cd, cnt) = cnt;
        if !cubes.contains(&cd) {
            if cnt == 3 {
                new.push(cd);
            }
        } else if cnt == 2 || cnt == 3 {
            new.push(cd);
        }
    }
    cubes.clear();
    cubes.extend(new);
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cubes.insert((x as isize, y as isize, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        it2(&mut cubes);
    }
    cubes.len()
}
