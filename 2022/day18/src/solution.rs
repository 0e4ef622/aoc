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
    let maxx = cubes.iter().map(|c| c.0).max().unwrap()+2;
    let maxy = cubes.iter().map(|c| c.1).max().unwrap()+2;
    let maxz = cubes.iter().map(|c| c.2).max().unwrap()+2;
    let mut sfa = 0;
    for &(x,y,z) in &cubes {
        if !cubes.contains(&(x+1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x-1,y,z)) { sfa+=1; }
        if !cubes.contains(&(x,y+1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y-1,z)) { sfa+=1; }
        if !cubes.contains(&(x,y,z+1)) { sfa+=1; }
        if !cubes.contains(&(x,y,z-1)) { sfa+=1; }
    }
    let mut queue = VecDeque::new();
    queue.push_back((-1,-1,-1));
    cubes.insert((-1,-1,-1));
    while let Some((x,y,z)) = queue.pop_front() {
        for (dx,dy,dz) in [(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)] {
            let nx = x+dx;
            let ny = y+dy;
            let nz = z+dz;
            if !cubes.contains(&(nx, ny, nz))
                && (-1..maxx).contains(&nx)
                && (-1..maxy).contains(&ny)
                && (-1..maxz).contains(&nz)
            {
                cubes.insert((nx,ny,nz));
                queue.push_back((nx,ny,nz));
            }
        }
    }
    let mut notcubes = HashSet::new();
    for x in -1..maxx {
        for y in -1..maxy {
            for z in -1..maxz {
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
