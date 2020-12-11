use std::collections::*;
use rand::random;
use serde_scan::scan as s;
fn iterate(grid: &Vec<Vec<u8>>, grid2: &mut Vec<Vec<u8>>) {
        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1 {
                let mut cnt_adj = 0;
                for dx in -1i32..=1 {
                    for dy in -1i32..=1 {
                        if dy == 0 && dx == 0 { continue; }
                        let y = (y as i32 + dy) as usize;
                        let x = (x as i32 + dx) as usize;
                        if grid[y][x] == b'#' {
                            cnt_adj += 1;
                        }
                    }
                }
                if grid[y][x] == b'L' && cnt_adj == 0 {
                    grid2[y][x] = b'#';
                } else if grid[y][x] == b'#' && cnt_adj >= 4 {
                    grid2[y][x] = b'L';
                } else {
                    grid2[y][x] = grid[y][x];
                }
            }
        }
}
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut grid = input.lines().map(|x| { let mut v = x.as_bytes().to_vec(); v.insert(0, b'.'); v.push(b'.'); v}).collect::<Vec<_>>();
    grid.insert(0, vec![b'.'; grid[0].len()]);
    grid.push(vec![b'.'; grid[0].len()]);
    let mut grid2 = grid.clone();
    // loop {
    loop {
        iterate(&grid, &mut grid2);
        if grid == grid2 { break; }
        std::mem::swap(&mut grid, &mut grid2);
    }
    grid.iter().flatten().filter(|&&x| x == b'#').count()
}

fn iterate2(grid: &Vec<Vec<u8>>, grid2: &mut Vec<Vec<u8>>) {
        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1 {
                let mut cnt_adj = 0;
                for dx in -1i32..=1 {
                    for dy in -1i32..=1 {
                        if dy == 0 && dx == 0 { continue; }
                        let mut y = y;
                        let mut x = x;
                        loop {
                            y = (y as i32 + dy) as usize;
                            x = (x as i32 + dx) as usize;
                            match grid.get(y).and_then(|r| r.get(x)) {
                                Some(b'#') => {
                                    cnt_adj += 1;
                                    break;
                                }
                                Some(b'L') | None => break,
                                _ => (),
                            }
                        }
                    }
                }
                if grid[y][x] == b'L' && cnt_adj == 0 {
                    grid2[y][x] = b'#';
                } else if grid[y][x] == b'#' && cnt_adj >= 5 {
                    grid2[y][x] = b'L';
                } else {
                    grid2[y][x] = grid[y][x];
                }
            }
        }
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut grid = input.lines().map(|x| { let mut v = x.as_bytes().to_vec(); v.insert(0, b'.'); v.push(b'.'); v}).collect::<Vec<_>>();
    grid.insert(0, vec![b'.'; grid[0].len()]);
    grid.push(vec![b'.'; grid[0].len()]);
    let mut grid2 = grid.clone();
    loop {
        iterate2(&grid, &mut grid2);
        if grid == grid2 { break; }
        std::mem::swap(&mut grid, &mut grid2);
    }
    grid.iter().flatten().filter(|&&x| x == b'#').count()
}
