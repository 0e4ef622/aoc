use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let cubes = input.lines().map(|line| {
        let mut it = line.split(',').map(|x| x.parse::<i64>().unwrap());
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }).cs();
    let mut sfa = 0;
    for &(x,y,z) in &cubes {
        if !cubes.contains(&(x+1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x-1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x,y+1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y-1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y,z+1)) { sfa+=1; }
        if !cubes.contains(&(x,y,z-1)) { sfa+=1; }
    }
    sfa
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut cubes = input.lines().map(|line| {
        let mut it = line.split(',').map(|x| x.parse::<i64>().unwrap());
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }).collect::<HashSet<_>>();
    let mut sfa = 0;
    for &(x,y,z) in &cubes {
        if !cubes.contains(&(x+1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x-1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x,y+1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y-1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y,z+1)) { sfa+=1; }
        if !cubes.contains(&(x,y,z-1)) { sfa+=1; }
    }
    let mut inner = 100*100*100;
    let mut queue = VecDeque::new();
    queue.push_back((0,0,0));
    cubes.insert((0,0,0));
    while let Some((x,y,z)) = queue.pop_front() {
        inner-=1;
        for (dx,dy,dz) in [(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)] {
            if !cubes.contains(&(x+dx,y+dy,z+dz)) && x+dx < 100 && x+dx >= 0
                && y+dy < 100 && y+dy >= 0
                && z+dz < 100 && z+dz >= 0
            {
                cubes.insert((x+dx,y+dy,z+dz));
                queue.push_back((x+dx,y+dy,z+dz));
            }
        }
    }
    let mut notcubes = HashSet::new();
    for x in 0..100 {
        for y in 0..100 {
            for z in 0..100 {
                if !cubes.contains(&(x,y,z)) {
                    notcubes.insert((x,y,z));
                }
            }
        }
    }
    for &(x,y,z) in &notcubes {
        if cubes.contains(&(x+1,y,z)) { sfa-=1; }
        if cubes.contains(&(x-1,y,z)) { sfa-=1; }
        if cubes.contains(&(x,y+1,z)) { sfa-=1; }
        if cubes.contains(&(x,y-1,z)) { sfa-=1; }
        if cubes.contains(&(x,y,z+1)) { sfa-=1; }
        if cubes.contains(&(x,y,z-1)) { sfa-=1; }
    }
    sfa
}
