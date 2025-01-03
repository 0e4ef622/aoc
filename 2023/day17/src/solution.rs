use std::collections::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Dir {
    U,D,L,R
}
fn mv(x: i32, y: i32, d: Dir) -> (i32, i32) {
    match d {
        Dir::U => (x, y-1),
        Dir::D => (x, y+1),
        Dir::L => (x-1, y),
        Dir::R => (x+1, y),
    }
}
fn left(d: Dir) -> Dir {
    match d {
        Dir::U => L,
        Dir::D => R,
        Dir::L => D,
        Dir::R => U,
    }
}
fn right(d: Dir) -> Dir {
    match d {
        Dir::U => R,
        Dir::D => L,
        Dir::L => U,
        Dir::R => D,
    }
}
use Dir::*;

struct CustomHeap<T> {
    q: Vec<Vec<T>>,
    qi: usize,
    len: usize,
}

impl<T> CustomHeap<T> {
    fn new(len: usize) -> Self {
        let mut q = Vec::new();
        q.resize_with(len, Vec::new);
        Self {
            q,
            qi: 0,
            len: 0,
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            while self.q[self.qi].len() == 0 {
                self.qi += 1;
            }
            self.q[self.qi].pop()
        }
    }

    fn push(&mut self, i: i32, v: T) {
        self.q[i as usize].push(v);
        self.len += 1;
    }
}

pub fn solve<const TURN_MIN: i32, const STRAIGHT_MAX: i32>(input: &str) -> i32 {
    let g = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let w = g[0].len() as i32;
    let h = g.len() as i32;
    let mut q = CustomHeap::new(2048);
    q.push(0, (0i32, (0i32, 0i32), 0, R));
    q.push(0, (0i32, (0i32, 0i32), 0, D));
    let mut v = vec![false; (w*h*4*(STRAIGHT_MAX+1)) as usize];
    let bck = |(x,y)| x>=0 && x<w && y>=0 && y<h;
    let mut ans = i32::MAX;
    let mut ac = 0;
    while let Some((mut d, (x, y), c, dir)) = q.pop() {
        if v[(dir as i32 + 4*(c + (STRAIGHT_MAX+1)*(x + w*y))) as usize] { continue; }
        v[(dir as i32 + 4*(c + (STRAIGHT_MAX+1)*(x + w*y))) as usize] = true;
        d += (g[y as usize][x as usize] - b'0') as i32;
        if x == w-1 && y == h-1 && c >= TURN_MIN {
            ans = d;
            break;
        }
        let r = mv(x,y,right(dir));
        let l = mv(x,y,left(dir));
        let f = mv(x,y,dir);
        if c >= TURN_MIN && bck(r) && !v[(right(dir) as i32 + 4*(1 + (STRAIGHT_MAX+1)*(r.0 + w*r.1))) as usize] { q.push(d, (d, r, 1, right(dir))); }
        if c >= TURN_MIN && bck(l) && !v[(left(dir) as i32 + 4*(1 + (STRAIGHT_MAX+1)*(l.0 + w*l.1))) as usize] { q.push(d, (d, l, 1, left(dir))); }
        if c < STRAIGHT_MAX && bck(f) && !v[(dir as i32 + 4*(c+1 + (STRAIGHT_MAX+1)*(f.0 + w*f.1))) as usize] { q.push(d, (d, f, c+1, dir)); }
    }
    ans - (g[0][0] - b'0') as i32
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve::<0, 3>(input)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve::<4, 10>(input)
}
pub fn run(input: &str) -> impl std::fmt::Display {
    part2(input)
}
