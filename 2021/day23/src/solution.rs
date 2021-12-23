use itertools::{iproduct, Itertools};
use rand::random;
use std::{cmp::Reverse, collections::*};
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
            s[i as usize][j as usize] = v as u8 + b'A';
        }
        for line in s {
            eprintln!("{}", std::str::from_utf8(&line).unwrap());
        }
    }
}

static HALLS: [(i8, i8); 7] = [(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];

static COST: [i64; 4] = [1, 10, 100, 1000];

fn has_path(queue_item: &QueueItem, start: (i8, i8), end: (i8, i8)) -> bool {
    assert!(start.0 == 1 || end.0 == 1);
    if start.0 == 1 {
        (start.1.min(end.1)..=start.1.max(end.1))
            .filter(|&c| queue_item.pending.contains_key(&(1, c)))
            .count()
            == 1
            && (1..=end.0)
                .filter(|&r| queue_item.pending.contains_key(&(r, end.1)))
                .count()
                == 0
    } else {
        (1..=start.0)
            .filter(|&r| queue_item.pending.contains_key(&(r, start.1)))
            .count()
            == 1
            && (start.1.min(end.1)..=start.1.max(end.1))
                .filter(|&c| queue_item.pending.contains_key(&(1, c)))
                .count()
                == 0
    }
}

fn dijkstra(initial: QueueItem) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut enqueue = |q: &mut BinaryHeap<_>, mut qi: QueueItem, v: &HashSet<_>| {
        if !v.contains(&qi) {
            q.push(Reverse(qi));
        }
    };
    queue.push(Reverse(initial));
    while queue.len() > 0 {
        let mut qi = queue.pop().unwrap().0;
        if visited.contains(&qi) {
            continue;
        }
        visited.insert(qi.clone());

        if qi.complete == [0; 4] {
            return qi.cost;
        }

        // Iterate over all amphipods that haven't gotten into their rooms yet.
        for (&(r, c), &ty) in &qi.pending {
            // If this amphipod is in the hallway
            if r == 1 {
                // Calculate room coordinates
                let t = (qi.complete[ty as usize] + 1, ty * 2 + 3);
                // If it's reachable, go to it
                if has_path(&qi, (r, c), t) {
                    let mut qi2 = qi.clone();
                    qi2.pending.remove(&(r, c)).unwrap();
                    qi2.complete[ty as usize] -= 1;
                    let cost = (r - t.0).abs() + (c - t.1).abs();
                    qi2.cost += COST[ty as usize] * (cost as i64);
                    enqueue(&mut queue, qi2, &visited);
                }
            } else {
                // Amphipod is in it's starting room, needs to go out into the hall
                for h in HALLS {
                    if has_path(&qi, (r, c), h) {
                        let mut qi2 = qi.clone();
                        qi2.pending.remove(&(r, c)).unwrap();
                        qi2.pending.insert(h, ty);
                        let cost = (r - h.0).abs() + (c - h.1).abs();
                        qi2.cost += COST[ty as usize] * (cost as i64);
                        enqueue(&mut queue, qi2, &visited);
                    }
                }
            }
        }
    }
    panic!("no solution");
    0
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut qi = QueueItem::default();
    qi.complete = [2; 4];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if (b'A'..=b'D').contains(&c) {
                qi.pending.insert((i as _, j as _), (c - b'A') as _);
            }
        }
    }

    dijkstra(qi)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().cv();
    lines.splice(3..3, ["  #D#C#B#A#", "  #D#B#A#C#"]);
    let mut qi = QueueItem::default();
    qi.complete = [4; 4];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if (b'A'..=b'D').contains(&c) {
                qi.pending.insert((i as _, j as _), (c - b'A') as _);
            }
        }
    }

    dijkstra(qi)
}
