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

    for i in 0..=h { eprintln!() }

    let mut v = vec![vec![false; w]; h];
    let mut q = BinaryHeap::new();
    q.push((-heur(0,0),0,0));
    loop {
        let (r, i, j) = q.pop().unwrap();
        visualize_visits(&v, &q, [w,h,i,j], &cost);
        let r = -r - heur(i, j);
        if i == h-1 && j == w-1 {
            return r;
        }
        if v[i][j] { continue; }
        v[i][j] = true;

        if i > 0   && !v[i-1][j] { q.push((-r - cost(i-1, j) - heur(i-1, j), i-1, j)); }
        if i < h-1 && !v[i+1][j] { q.push((-r - cost(i+1, j) - heur(i+1, j), i+1, j)); }
        if j > 0   && !v[i][j-1] { q.push((-r - cost(i, j-1) - heur(i, j-1), i, j-1)); }
        if j < w-1 && !v[i][j+1] { q.push((-r - cost(i, j+1) - heur(i, j+1), i, j+1)); }
    }
}

fn visualize_visits<F>(v: &[Vec<bool>], q: &BinaryHeap<(i64, usize, usize)>, [w, h, i, j]: [usize; 4], cost: F)
where
    F: Fn(usize, usize) -> i64,
{
    use std::io::Write;
    use termcolor::{ColorChoice, StandardStream, WriteColor};

    eprint!("[{}A", h);

    let mut stdout = StandardStream::stderr(ColorChoice::Auto);
    for y in 0..h {
        for x in 0..w {
            let mut color = ColorSpec::new();
            if y == i && x == j {
                color.set_bg(Some(Color::White));
                color.set_fg(Some(Color::Black));
            } else if v[y][x] {
                color.set_fg(Some(Color::Red));
            } else if q.iter().find(|(_, i, j)| y==*i && x==*j).is_some() {
                color.set_fg(Some(Color::Green));
            }
            stdout.set_color(&color).unwrap();
            write!(&mut stdout, "{}", cost(y, x));
            stdout.reset().unwrap();
        }
        writeln!(&mut stdout);
    }
    // std::thread::sleep(Duration::from_millis(50));
}
