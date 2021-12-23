use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Default, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
struct QueueItem {
    cost: i64,
    pending: BTreeMap<(i8, i8), i8>,
    complete: [i8; 4],
}

impl QueueItem {
    fn print(&self) {
        let mut s = [
*b"#############",
*b"#...........#",
*b"###.#.#.#.###",
*b"  #.#.#.#.#  ",
*b"  #########  ",
        ];
        for (&(i, j), &v) in &self.pending {
            s[i as usize][j as usize] = v as u8+b'A';
        }
        for line in s {
            eprintln!("{}", std::str::from_utf8(&line).unwrap());
        }
    }
}

static HALLS: [(i8, i8); 7] = [
    (1,1),
    (1,2),
    (1,4),
    (1,6),
    (1,8),
    (1,10),
    (1,11),
];

static COST: [i64; 4] = [1, 10, 100, 1000];

fn has_path(queue_item: &QueueItem, start: (i8, i8), end: (i8, i8)) -> bool {
    assert!(start.0 == 1 || end.0 == 1);
    // eprintln!("has_path({:?}, {:?})", start, end);
    if start.0 == 1 {
        (start.1.min(end.1)..=start.1.max(end.1))
            .filter(|&c| queue_item.pending.contains_key(&(1, c)))
            .count() == 1 &&
        (1..=end.0)
            .filter(|&r| queue_item.pending.contains_key(&(r, end.1)))
            .count() == 0
    } else {
        (1..=start.0)
            .filter(|&r| queue_item.pending.contains_key(&(r, start.1)))
            .count() == 1 &&
        (start.1.min(end.1)..=start.1.max(end.1))
            .filter(|&c| queue_item.pending.contains_key(&(1, c)))
            .count() == 0
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut qi = QueueItem::default();
    qi.complete = [2; 4];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if (b'A'..=b'D').contains(&c) {
                qi.pending.insert((i as _, j as _), (c-b'A') as _);
            }
        }
    }

    let mut v = HashSet::new();
    let mut q = BinaryHeap::new();
    let mut enq = |q: &mut BinaryHeap<_>, mut qi: QueueItem, v: &HashSet<_>| {
        if !v.contains(&qi) {
            qi.cost *= -1;
            q.push(qi);
        }
    };
    q.push(qi);
    while q.len() > 0 {
        let mut qi = q.pop().unwrap();
        qi.cost *= -1;
        if v.contains(&qi) { continue; }
        v.insert(qi.clone());
        // qi.print();
        // eprintln!("{}", q.len());
        if qi.complete == [0; 4] {
            return qi.cost;
        }
        for it in &qi.pending {
            if it.0.0 == 1 {
                let t = (qi.complete[*it.1 as usize]+1, it.1*2+3);
                if has_path(&qi, *it.0, t) {
                    let mut qi2 = qi.clone();
                    qi2.pending.remove(it.0).unwrap();
                    qi2.complete[*it.1 as usize] -= 1;
                    let cost = (it.0.0-t.0).abs() + (it.0.1-t.1).abs();
                    qi2.cost += COST[*it.1 as usize]*(cost as i64);
                    enq(&mut q, qi2, &v);
                }
            } else {
                for h in HALLS {
                    if has_path(&qi, *it.0, h) {
                        let mut qi2 = qi.clone();
                        qi2.pending.remove(it.0).unwrap();
                        qi2.pending.insert(h, *it.1);
                        let cost = (it.0.0-h.0).abs() + (it.0.1-h.1).abs();
                        qi2.cost += COST[*it.1 as usize]*(cost as i64);
                        enq(&mut q, qi2, &v);
                    }
                }
            }
        }
    }
    panic!("no solution");
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().cv();
    lines.splice(3..3, ["  #D#C#B#A#", "  #D#B#A#C#"]);
    let mut qi = QueueItem::default();
    qi.complete = [4; 4];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if (b'A'..=b'D').contains(&c) {
                qi.pending.insert((i as _, j as _), (c-b'A') as _);
            }
        }
    }

    let mut v = HashSet::new();
    let mut q = BinaryHeap::new();
    let mut enq = |q: &mut BinaryHeap<_>, mut qi: QueueItem, v: &HashSet<_>| {
        if !v.contains(&qi) {
            qi.cost *= -1;
            q.push(qi);
        }
    };
    q.push(qi);
    while q.len() > 0 {
        let mut qi = q.pop().unwrap();
        qi.cost *= -1;
        if v.contains(&qi) { continue; }
        v.insert(qi.clone());
        // qi.print();
        // eprintln!("{}", q.len());
        if qi.complete == [0; 4] {
            return qi.cost;
        }
        for it in &qi.pending {
            if it.0.0 == 1 {
                let t = (qi.complete[*it.1 as usize]+1, it.1*2+3);
                if has_path(&qi, *it.0, t) {
                    let mut qi2 = qi.clone();
                    qi2.pending.remove(it.0).unwrap();
                    qi2.complete[*it.1 as usize] -= 1;
                    let cost = (it.0.0-t.0).abs() + (it.0.1-t.1).abs();
                    qi2.cost += COST[*it.1 as usize]*(cost as i64);
                    enq(&mut q, qi2, &v);
                }
            } else {
                for h in HALLS {
                    if has_path(&qi, *it.0, h) {
                        let mut qi2 = qi.clone();
                        qi2.pending.remove(it.0).unwrap();
                        qi2.pending.insert(h, *it.1);
                        let cost = (it.0.0-h.0).abs() + (it.0.1-h.1).abs();
                        qi2.cost += COST[*it.1 as usize]*(cost as i64);
                        enq(&mut q, qi2, &v);
                    }
                }
            }
        }
    }
    panic!("no solution");
    0
}
