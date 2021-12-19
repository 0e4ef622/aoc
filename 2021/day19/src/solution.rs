use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

fn rot(a: &mut [[i16; 3]], dir: &str) {
    match dir {
        "xy" => a.iter_mut().for_each(|v| (v[0], v[1]) = (v[1], -v[0])),
        "yz" => a.iter_mut().for_each(|v| (v[1], v[2]) = (v[2], -v[1])),
        "-yz" => a.iter_mut().for_each(|v| (v[1], v[2]) = (-v[2], v[1])),
        "xz" => a.iter_mut().for_each(|v| (v[0], v[2]) = (v[2], -v[0])),
        _ => todo!(),
    }
}

struct Rots<'a> {
    a: &'a mut [[i16; 3]],
    c: u8,
}

impl<'a> Rots<'a> {
    fn new(a: &'a mut [[i16; 3]]) -> Self {
        Self { a, c: 0 }
    }

    fn rot(&mut self) -> bool {
        self.c += 1;
        match self.c {
            1 => (),
            2 => rot(self.a, "xy"),
            3 => rot(self.a, "xy"),
            4 => rot(self.a, "xy"),

            5 => rot(self.a, "xz"),
            6 => rot(self.a, "xy"),
            7 => rot(self.a, "xy"),
            8 => rot(self.a, "xy"),

            9 => rot(self.a, "xz"),
            10 => rot(self.a, "xy"),
            11 => rot(self.a, "xy"),
            12 => rot(self.a, "xy"),

            13 => rot(self.a, "yz"),
            14 => rot(self.a, "xy"),
            15 => rot(self.a, "xy"),
            16 => rot(self.a, "xy"),

            17 => rot(self.a, "-yz"),
            18 => rot(self.a, "xy"),
            19 => rot(self.a, "xy"),
            20 => rot(self.a, "xy"),

            21 => rot(self.a, "-yz"),
            22 => rot(self.a, "xy"),
            23 => rot(self.a, "xy"),
            24 => rot(self.a, "xy"),
            _ => return false,
        }
        true
    }
}

fn merge(a: &[[i16; 3]], b: &[[i16; 3]]) -> Option<[i16; 3]> {
    for (aa, bb) in iproduct!(a, b) {
        let d = [aa[0] - bb[0], aa[1] - bb[1], aa[2] - bb[2]];
        let bset = b.iter().map(|c| [c[0]+d[0], c[1]+d[1], c[2]+d[2]]);
        if bset.filter(|v| a.contains(v)).count() >= 12 {
            return Some(d);
        }
    }
    None
}

fn rotmerge(a: &[[i16; 3]], b: &mut [[i16; 3]], calc: bool) -> Option<[i16; 3]> {
    let mut rots = Rots::new(b);
    while rots.rot() {
        if let Some(d) = merge(a, rots.a) {
            if calc {
                for v in b { v[0] += d[0]; v[1] += d[1]; v[2] += d[2]; }
            }
            return Some(d);
        }
    }
    None
}

fn c3<T>(mut i: impl Iterator<Item = T>) -> [T; 3] {
    [i.next().unwrap(), i.next().unwrap(), i.next().unwrap()]
}

fn idx2<T>(s: &mut [T], i: usize, j: usize) -> [&mut T; 2] {
    debug_assert_ne!(i, j);
    let ptr = s.as_mut_ptr();
    unsafe {
        [&mut *ptr.offset(i as isize), &mut *ptr.offset(j as isize)]
    }
}

fn dist(a: [i16; 3], b: [i16; 3]) -> u64 {
    let (x, y, z) = (a[0] as u64-b[0] as u64, a[1] as u64-b[1] as u64, a[2] as u64-b[2] as u64);
    x*x + y*y + z*z
}

fn maybe_overlap(a: &[[i16; 3]], b: &[[i16; 3]]) -> bool {
    let mut ads = HashSet::default();
    for i in 0..a.len() {
        for j in i+1..a.len() {
            ads.insert(dist(a[i], a[j]));
        }
    }
    let mut bds = HashSet::default();
    for i in 0..b.len() {
        for j in i+1..b.len() {
            bds.insert(dist(b[i], b[j]));
        }
    }
    ads.intersection(&bds).count() >= 12*11/2
}

fn dfs(sc: &mut [Vec<[i16; 3]>], s: &mut Vec<[i16; 3]>, f: &mut HashSet<[i16; 3]>, g: &[Vec<usize>], i: usize, p: usize) {
    let [a, b] = idx2(sc, i, p);
    s.push(rotmerge(b, a, true).unwrap());
    f.extend(sc[i].iter().copied());
    for &ch in &g[i] {
        if ch == p { continue; }
        dfs(sc, s, f, g, ch, i);
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sc = input.split("\n\n").map(|sect| sect.lines().skip(1).map(|s| c3(s.split(',').map(|v| v.parse::<i16>().unwrap()))).cv().chd(<[_]>::sort_unstable)).cv();
    let mut g = vec![vec![]; sc.len()];
    let mut s = Dsu::new(sc.len());
    let mut cn = sc.len()-1;
    'a: for i in 0..sc.len() {
        for j in i+1..sc.len() {
            if s.find(i) == s.find(j) { continue; }
            if !maybe_overlap(&sc[i], &sc[j]) { continue; }
            let [a, b] = idx2(&mut sc, i, j);
            if let Some(_o) = rotmerge(a, b, false) {
                cn -= 1;
                s.merge(i, j);
                g[i].push(j);
                g[j].push(i);
                if cn == 0 { break; }
            }
        }
    }
    let mut f = HashSet::default();
    dfs(&mut sc, &mut vec![], &mut f, &g, 0, 0);
    f.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sc = input.split("\n\n").map(|sect| sect.lines().skip(1).map(|s| c3(s.split(',').map(|v| v.parse::<i16>().unwrap()))).cv().chd(<[_]>::sort_unstable)).cv();
    let mut g = vec![vec![]; sc.len()];
    let mut s = Dsu::new(sc.len());
    let mut cn = sc.len()-1;
    'a: for i in 0..sc.len() {
        for j in i+1..sc.len() {
            if s.find(i) == s.find(j) { continue; }
            if !maybe_overlap(&sc[i], &sc[j]) { continue; }
            let [a, b] = idx2(&mut sc, i, j);
            if let Some(_o) = rotmerge(a, b, false) {
                cn -= 1;
                s.merge(i, j);
                g[i].push(j);
                g[j].push(i);
                if cn == 0 { break; }
            }
        }
    }
    let mut f = HashSet::default();
    let mut sp = vec![];
    dfs(&mut sc, &mut sp, &mut f, &g, 0, 0);
    iproduct!(&sp, &sp).map(|(a, b)| (a[0]-b[0]).abs() + (a[1]-b[1]).abs() + (a[2]-b[2]).abs()).max().unwrap()
}
