use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn neighbors(c: u8, x: i64, y: i64) -> [[i64; 2]; 2] {
    match c {
        b'|' => [[x, y+1], [x, y-1]],
        b'-' => [[x+1, y], [x-1, y]],
        b'L' => [[x, y-1], [x+1, y]],
        b'J' => [[x, y-1], [x-1, y]],
        b'7' => [[x, y+1], [x-1, y]],
        b'F' => [[x, y+1], [x+1, y]],
        _ => [[-1, -1], [-1, -1]],
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|x| x.as_bytes()).cv();
    let h = grid.len();
    let w = grid[0].len();
    let mut g = HashMap::new();
    let mut s = [0, 0];
    for i in 0..h {
        for j in 0..w {
            g.insert((i as i64, j as i64), grid[i][j]);
            if grid[i][j] == b'S' {
                s = [j as i64, i as i64];
            }
        }
    }

    let [sx, sy] = s;
    let mut q = VecDeque::new();
    if neighbors(g[&(sy+1, sx)], sx, sy+1).contains(&s) { q.push_back((1, sx, sy+1)); }
    if neighbors(g[&(sy-1, sx)], sx, sy-1).contains(&s) { q.push_back((1, sx, sy-1)); }
    if neighbors(g[&(sy, sx+1)], sx+1, sy).contains(&s) { q.push_back((1, sx+1, sy)); }
    if neighbors(g[&(sy, sx-1)], sx-1, sy).contains(&s) { q.push_back((1, sx-1, sy)); }
    let mut ans = 0;
    let mut vis = HashSet::new();
    vis.insert((sx, sy));
    while let Some((d, x, y)) = q.pop_front() {
        if vis.contains(&(x, y)) { continue; }
        vis.insert((x, y));
        ans = d;
        let ne = neighbors(g[&(y, x)], x, y);
        if ne[0][0] != -1 {
            q.push_back((d+1, ne[0][0], ne[0][1]));
            q.push_back((d+1, ne[1][0], ne[1][1]));
        }
    }

    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = input.lines().map(|x| x.as_bytes()).cv();
    let h = grid.len();
    let w = grid[0].len();
    let mut g = HashMap::new();
    let mut s = [0, 0];
    for i in 0..h {
        for j in 0..w {
            g.insert((i as i64, j as i64), grid[i][j]);
            if grid[i][j] == b'S' {
                s = [j as i64, i as i64];
            }
        }
    }

    let [sx, sy] = s;
    let mut q = VecDeque::new();
    if neighbors(g[&(sy+1, sx)], sx, sy+1).contains(&s) { q.push_back((1, sx, sy+1, sx, sy)); }
    if neighbors(g[&(sy-1, sx)], sx, sy-1).contains(&s) { q.push_back((1, sx, sy-1, sx, sy)); }
    if neighbors(g[&(sy, sx+1)], sx+1, sy).contains(&s) { q.push_back((1, sx+1, sy, sx, sy)); }
    if neighbors(g[&(sy, sx-1)], sx-1, sy).contains(&s) { q.push_back((1, sx-1, sy, sx, sy)); }
    q.pop_back();
    let mut ans = 0;
    let mut vis = HashMap::new();
    let mut area = 0;
    let mut last = (0, 0);
    vis.insert((sx, sy), 0);
    while let Some((d, x, y, px, py)) = q.pop_front() {
        if vis.contains_key(&(x, y)) { continue; }
        vis.insert((x, y), d);
        area += x*py - px*y;
        ans = d;
        let ne = neighbors(g[&(y, x)], x, y);
        if ne[0][0] != -1 {
            q.push_back((d+1, ne[0][0], ne[0][1], x, y));
            q.push_back((d+1, ne[1][0], ne[1][1], x, y));
        }
        last = (x, y);
    }
    area += sx*last.1 - sy*last.0;
    area /= 2;
    area.abs() + 1 - vis.len() as i64/2
    // for i in 0..h {
    //     for j in 0..w {
    //         if vis.contains(&(j as i64, i as i64)) {
    //             print!("{}", g[&(i as i64, j as i64)] as char);
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

}
