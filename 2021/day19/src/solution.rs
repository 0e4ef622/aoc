use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
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
        let bset = b.iter().map(|c| [c[0] + d[0], c[1] + d[1], c[2] + d[2]]);
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
                for v in b {
                    v[0] += d[0];
                    v[1] += d[1];
                    v[2] += d[2];
                }
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
    unsafe { [&mut *ptr.offset(i as isize), &mut *ptr.offset(j as isize)] }
}

fn dist(a: [i16; 3], b: [i16; 3]) -> i64 {
    let (x, y, z) = (
        a[0] as i64 - b[0] as i64,
        a[1] as i64 - b[1] as i64,
        a[2] as i64 - b[2] as i64,
    );
    x * x + y * y + z * z
}

fn maybe_overlap(a: &[[i16; 3]], b: &[[i16; 3]]) -> Option<[Vec<[i16; 3]>; 2]> {
    #[derive(Eq)]
    struct Dist {
        dist: i64,
        p1: [i16; 3],
        p2: [i16; 3],
    }

    impl PartialEq for Dist {
        fn eq(&self, other: &Self) -> bool {
            self.dist == other.dist
        }
    }
    impl std::hash::Hash for Dist {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.dist.hash(state);
        }
    }

    let mut ads = HashSet::default();
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            ads.insert(Dist {
                dist: dist(a[i], a[j]),
                p1: a[i],
                p2: a[j],
            });
        }
    }

    let mut bds = HashSet::default();
    for i in 0..b.len() {
        for j in i + 1..b.len() {
            bds.insert(Dist {
                dist: dist(b[i], b[j]),
                p1: b[i],
                p2: b[j],
            });
        }
    }
    let intersection = ads.intersection(&bds).cv();
    if intersection.len() < 12 * 11 / 2 {
        return None;
    }
    let mut ads = intersection.iter().flat_map(|v| [v.p1, v.p2]).cv();
    let mut bds = intersection
        .into_iter()
        .flat_map(|e| bds.get(e))
        .flat_map(|v| [v.p1, v.p2])
        .cv();

    ads.sort_unstable();
    ads.dedup();
    bds.sort_unstable();
    bds.dedup();

    if ads.len() < 12 || bds.len() < 12 {
        return None;
    }

    Some([ads, bds])
}

fn dfs(
    scanners: &mut [Vec<[i16; 3]>],
    scanner_pos: &mut Vec<[i16; 3]>,
    beacons: &mut HashSet<[i16; 3]>,
    graph: &[Vec<usize>],
    current: usize,
    previous: usize,
) {
    if current != previous {
        let [a, b] = idx2(scanners, current, previous);
        scanner_pos.push(rotmerge(b, a, true).unwrap());
    }
    beacons.extend(scanners[current].iter().copied());
    for &ch in &graph[current] {
        if ch == previous {
            continue;
        }
        dfs(scanners, scanner_pos, beacons, graph, ch, current);
    }
}

fn parse_input(input: &str) -> Vec<Vec<[i16; 3]>> {
    let mut sc = input
        .split("\n\n")
        .map(|sect| {
            sect.lines()
                .skip(1)
                .map(|s| c3(s.split(',').map(|v| v.parse::<i16>().unwrap())))
                .cv()
                .chd(<[_]>::sort_unstable)
        })
        .cv();
    sc
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut scanners = parse_input(input);
    let mut graph = vec![vec![]; scanners.len()];
    let mut dsu = Dsu::new(scanners.len());
    let mut edges_left = scanners.len() - 1;
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            if dsu.find(i) == dsu.find(j) {
                continue;
            }
            if let Some([a, mut b]) = maybe_overlap(&scanners[i], &scanners[j]) {
                if let Some(_o) = rotmerge(&a, &mut b, false) {
                    edges_left -= 1;
                    // eprintln!("{} remaining", cn);
                    dsu.merge(i, j);
                    graph[i].push(j);
                    graph[j].push(i);
                    if edges_left == 0 {
                        break;
                    }
                }
            }
        }
    }
    let mut f = HashSet::default();
    dfs(&mut scanners, &mut vec![], &mut f, &graph, 0, 0);
    f.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut scanners = parse_input(input);
    let mut graph = vec![vec![]; scanners.len()];
    let mut dsu = Dsu::new(scanners.len());
    let mut edges_left = scanners.len() - 1;
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            if dsu.find(i) == dsu.find(j) {
                continue;
            }
            if let Some([a, mut b]) = maybe_overlap(&scanners[i], &scanners[j]) {
                if let Some(_o) = rotmerge(&a, &mut b, false) {
                    edges_left -= 1;
                    // eprintln!("{} remaining", cn);
                    dsu.merge(i, j);
                    graph[i].push(j);
                    graph[j].push(i);
                    if edges_left == 0 {
                        break;
                    }
                }
            }
        }
    }
    let mut f = HashSet::default();
    let mut sp = vec![];
    dfs(&mut scanners, &mut sp, &mut f, &graph, 0, 0);
    iproduct!(&sp, &sp)
        .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
        .max()
        .unwrap()
}
