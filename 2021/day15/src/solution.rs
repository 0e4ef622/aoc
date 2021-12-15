use std::{collections::*, time::Duration};
use rand::random;
use itertools::{iproduct, Itertools};
use termcolor::{ColorSpec, Color};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let g = input.lines().map(|l| l.bytes().map(|b| (b-b'0') as i64).cv()).cv();
    let w = g[0].len();
    let h = g.len();

    a_star(h, w, |i,j| g[i][j], |i,j| ((h-1-i) + (w-1-j)) as i64)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g = input.lines().map(|l| l.bytes().map(|b| (b-b'0') as i64).cv()).cv();
    let w = g[0].len();
    let h = g.len();

    let cost = |r: usize, c| (g[r%h][c%w] + (r/h) as i64 + (c/w) as i64 - 1) % 9 + 1;

    let w = w*5;
    let h = h*5;

    a_star(h, w, cost, |i,j| ((h-1-i) + (w-1-j)) as i64)
}

fn a_star<F, G>(h: usize, w: usize, cost: F, heur: G) -> i64
where
    F: Fn(usize, usize) -> i64,
    G: Fn(usize, usize) -> i64,
{

    let mut vv = VisViz::new(w, h);

    let mut v = vec![vec![None; w]; h];
    let mut q = BinaryHeap::new();
    q.push((-heur(0,0),0,0,0,0));
    loop {
        let (r, i, j, pi, pj) = q.pop().unwrap();
        let r = -r - heur(i, j);

        if v[i][j].is_some() { continue; }
        v[i][j] = Some((pi, pj));

        vv.viz(&v, &q, [i,j], &cost);
        if i == h-1 && j == w-1 {
            vv.finish(&v, [i, j], &cost);
            return r;
        }

        if i > 0   && v[i-1][j].is_none() { q.push((-r - cost(i-1, j) - heur(i-1, j), i-1, j, i, j)); }
        if i < h-1 && v[i+1][j].is_none() { q.push((-r - cost(i+1, j) - heur(i+1, j), i+1, j, i, j)); }
        if j > 0   && v[i][j-1].is_none() { q.push((-r - cost(i, j-1) - heur(i, j-1), i, j-1, i, j)); }
        if j < w-1 && v[i][j+1].is_none() { q.push((-r - cost(i, j+1) - heur(i, j+1), i, j+1, i, j)); }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VizState {
    Uninit,
    Unvisited,
    Visited,
    Queued,
    Path,
    Current,
}

impl VizState {
    fn color_spec(self) -> termcolor::ColorSpec {
        let mut c = termcolor::ColorSpec::new();
        match self {
            VizState::Uninit | VizState::Unvisited => (),
            VizState::Visited => { c.set_fg(Some(Color::Red)); },
            VizState::Queued => { c.set_fg(Some(Color::Green)); },
            VizState::Path => { c.set_bg(Some(Color::White)).set_fg(Some(Color::Black)); },
            VizState::Current => { c.set_bg(Some(Color::Green)).set_fg(Some(Color::Black)); },
            _ => todo!(),
        }
        c
    }
}

struct VisViz {
    w: usize,
    h: usize,
    prev: Vec<Vec<VizState>>,
}

impl VisViz {
    fn new(w: usize, h: usize) -> Self {
        for i in 0..=h { eprintln!() }
        eprint!("   ");
        Self { w, h, prev: vec![vec![VizState::Uninit; w]; h] }
    }
    fn viz<F>(&mut self, v: &[Vec<Option<(usize, usize)>>], q: &BinaryHeap<(i64, usize, usize, usize, usize)>, [i, j]: [usize; 2], cost: F)
    where
        F: Fn(usize, usize) -> i64,
    {
        let Self { w, h, .. } = *self;

        let mut path = HashSet::new();
        path.insert((0,0));
        for (mut y, mut x) in q.iter().map(|x| (x.3, x.4)).chain(v[i][j]) {
            while (y, x) != (0, 0) {
                path.insert((y, x));
                let (i, j) = v[y][x].unwrap();
                y = i;
                x = j;
            }
        }
        let q: HashSet<_> = q.iter().map(|x| (x.1, x.2)).collect::<HashSet<_>>().ch(|q| q.insert((i, j)));

        let mut new = vec![vec![VizState::Uninit; w]; h];
        for y in 0..h {
            for x in 0..w {
                new[y][x] = if (y, x) == (i, j) {
                    VizState::Current
                } else if path.contains(&(y, x)) {
                    VizState::Path
                } else if v[y][x].is_some() {
                    VizState::Visited
                } else if q.contains(&(y, x)) {
                    VizState::Queued
                } else {
                    VizState::Unvisited
                };
            }
        }
        self.update(new, cost);
        // std::thread::sleep(Duration::from_millis(20));
    }
    fn finish<F>(&mut self, v: &[Vec<Option<(usize, usize)>>], [mut i, mut j]: [usize; 2], cost: F)
    where
        F: Fn(usize, usize) -> i64,
    {
        let Self { w, h, .. } = *self;

        let mut path = HashSet::new();
        path.insert((0,0));
        while (i, j) != (0, 0) {
            path.insert((i, j));
            let (y, x) = v[i][j].unwrap();
            i = y;
            j = x;
        }

        let mut new = vec![vec![VizState::Visited; w]; h];
        for (y, x) in path {
            new[y][x] = VizState::Current;
        }
        self.update(new, cost);
        // std::thread::sleep(Duration::from_millis(20));
    }
    fn update<F>(&mut self, new: Vec<Vec<VizState>>, cost: F)
    where
        F: Fn(usize, usize) -> i64,
    {
        use std::io::Write;
        use termcolor::{ColorChoice, StandardStream, WriteColor};

        let mut out = StandardStream::stderr(ColorChoice::Auto);
        let Self { w, h, .. } = *self;
        let (mut cy, mut cx) = (h as i32, 0);
        for y in 0..h {
            for x in 0..w {
                if new[y][x] != self.prev[y][x] {
                    self.move_cursor(x as i32 - cx, y as i32 - cy);
                    let color = new[y][x].color_spec();
                    out.set_color(&color).unwrap();
                    write!(&mut out, "{}", cost(y, x));
                    cy = y as i32;
                    cx = x as i32 + 1;
                }
            }
        }
        out.reset().unwrap();
        self.move_cursor(-cx, h as i32 - cy);
        self.prev = new;
    }

    fn move_cursor(&self, x: i32, y: i32) {
        if x < 0 {
            eprint!("[{}D", -x);
        } else if x > 0 {
            eprint!("[{}C", x);
        }
        if y < 0 {
            eprint!("[{}A", -y);
        } else if y > 0 {
            eprint!("[{}B", y);
        }
        // std::thread::sleep(Duration::from_millis(100));
    }
}
