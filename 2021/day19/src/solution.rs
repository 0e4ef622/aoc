use itertools::{iproduct, Itertools};
use rand::random;
use std::collections::*;
use util::*;

type HashSet<T> = std::collections::HashSet<T, ahash::RandomState>;

fn rot(p: [i16; 3], r: [i16; 3]) -> [i16; 3] {
    [
        r[0].signum() * p[r[0].abs() as usize - 1],
        r[1].signum() * p[r[1].abs() as usize - 1],
        r[2].signum() * p[r[2].abs() as usize - 1],
    ]
}

fn invrot(r: [i16; 3]) -> [i16; 3] {
    let mut r2 = [0; 3];
    for i in 0..3 { r2[r[i].abs() as usize - 1] = r[i].signum() * (i + 1) as i16; }
    r2
}

fn unrot(p: [i16; 3], r: [i16; 3]) -> [i16; 3] {
    rot(p, invrot(r))
}

// Rotate, then translate
#[derive(Copy, Clone, Debug)]
struct Transform {
    rot: [i16; 3],
    trans: [i16; 3],
}

impl Transform {
    fn new() -> Self {
        Self { rot: [1, 2, 3], trans: [0; 3] }
    }
    fn then(&self, o: &Transform) -> Self {
        let mut trans = rot(self.trans, o.rot);
        trans[0] += o.trans[0];
        trans[1] += o.trans[1];
        trans[2] += o.trans[2];
        let rot = rot(self.rot, o.rot);
        Self { rot, trans }
    }
    fn inv(&self) -> Self {
        let mut trans = unrot(self.trans, self.rot);
        trans[0] *= -1;
        trans[1] *= -1;
        trans[2] *= -1;
        let rot = invrot(self.rot);
        Self { trans, rot }
    }

    fn apply(&self, mut p: [i16; 3]) -> [i16; 3] {
        p = rot(p, self.rot);
        p[0] += self.trans[0];
        p[1] += self.trans[1];
        p[2] += self.trans[2];
        p
    }
}

static ROTS: [[i16; 3]; 24] = [
    [1, 2, 3],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],

    [3, 2, -1],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],

    [3, 2, -1],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],

    [1, 3, -2],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],

    [1, -3, 2],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],

    [1, -3, 2],
    [2, -1, 3],
    [2, -1, 3],
    [2, -1, 3],
];
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

fn rotmerge(a: &[[i16; 3]], b: &mut [[i16; 3]]) -> Option<Transform> {
    let mut ro = [1, 2, 3];
    for &r in &ROTS {
        ro = rot(ro, r);
        b.iter_mut().for_each(|p| *p = rot(*p, r));
        if let Some(d) = merge(a, b) {
            let mut t = Transform { rot: [1,2,3], trans: d };
            b.iter_mut().for_each(|p| *p = t.apply(*p));
            t.rot = ro;
            return Some(t);
        }
    }
    None
}

fn collect3<T>(mut i: impl Iterator<Item = T>) -> [T; 3] {
    [i.next().unwrap(), i.next().unwrap(), i.next().unwrap()]
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
    let mut ads = intersection
        .iter()
        .flat_map(|e| ads.get(e))
        .flat_map(|v| [v.p1, v.p2]).cv();
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
    graph: &[Vec<(usize, Transform, bool)>],
    current: usize,
    previous: usize,
    update_beacon_positions: bool,
    t: Transform,
) {
    scanner_pos.push(t.trans);
    if current != previous && update_beacon_positions {
        scanners[current].iter_mut().for_each(|p| *p = t.apply(*p));
    }
    beacons.extend(scanners[current].iter().copied());
    for &(ch, tt, ck) in &graph[current] {
        if ch == previous {
            continue;
        }
        dfs(scanners, scanner_pos, beacons, graph, ch, current, update_beacon_positions, tt.then(&t));
    }
}

fn parse_input(input: &str) -> Vec<Vec<[i16; 3]>> {
    let mut sc = input
        .split("\n\n")
        .map(|sect| {
            sect.lines()
                .skip(1)
                .map(|s| collect3(s.split(',').map(|v| v.parse::<i16>().unwrap())))
                .cv()
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
                if let Some(t) = rotmerge(&a, &mut b) {
                    edges_left -= 1;
                    dsu.merge(i, j);
                    graph[i].push((j, t, true));
                    graph[j].push((i, t.inv(), false));
                    if edges_left == 0 {
                        break;
                    }
                }
            }
        }
    }
    let mut f = HashSet::default();
    dfs(&mut scanners, &mut vec![], &mut f, &graph, 0, 0, true, Transform::new());
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
                if let Some(t) = rotmerge(&a, &mut b) {
                    edges_left -= 1;
                    dsu.merge(i, j);
                    graph[i].push((j, t, true));
                    graph[j].push((i, t.inv(), false));
                    if edges_left == 0 {
                        break;
                    }
                }
            }
        }
    }
    let mut f = HashSet::default();
    let mut sp = vec![];
    dfs(&mut scanners, &mut sp, &mut f, &graph, 0, 0, false, Transform::new());
    iproduct!(&sp, &sp)
        .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
        .max()
        .unwrap()
}
