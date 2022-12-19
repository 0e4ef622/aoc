use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn sat(costs: &[[i64; 4]; 4], r: &[i64; 4], i: usize) -> bool {
    r[0] >= costs[i][0]
        && r[1] >= costs[i][1]
        && r[2] >= costs[i][2]
        && r[3] >= costs[i][3]
}

fn dijk(costs: &[[i64; 4]; 4]) -> i64 {
    let mut q = BinaryHeap::new();
    let mut v = HashSet::new();
    q.push((0, 1, [1,0,0,0], [0,0,0,0]));

    while let Some((c, t, b, r)) = q.pop() {
        if v.contains(&(b,r)) { continue; }
        v.insert((b, r));
        let mut r2 = r;
        for i in 0..4 {
            r2[i] += b[i];
        }
        if t == 24 {
            eprintln!("r2 = {r2:?}");
            eprintln!("b  = {b:?}");
            eprintln!("{c}");
            return r2[3];
        }
        for i in 0..4 {
            if sat(costs, &r, i) {
                let mut b = b;
                let mut r2 = r2;
                b[i] += 1;
                r2[0] -= costs[i][0];
                r2[1] -= costs[i][1];
                r2[2] -= costs[i][2];
                r2[3] -= costs[i][3];
                if !v.contains(&(b, r2)) {
                    if i == 3 {
                        q.push((c, t+1, b, r2));
                    } else {
                        q.push((c-(24-t), t+1, b, r2));
                    }
                }
            }
        }
        q.push((c-(24-t), t+1, b, r2));
    }
    -1
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut rid = HashMap::new();
    rid.insert("ore", 0);
    rid.insert("clay", 1);
    rid.insert("obsidian", 2);
    rid.insert("geode", 3);
    // let mut blueprints = vec![];
    let mut ans=0;
    for line in input.lines() {
        let (b, s) = line.split_once(": ").unwrap();
        let id = b.split_whitespace().cv()[1].parse::<i64>().unwrap();
        let sentences = s.split(". ");
        let robot_costs: [[i64; 4]; 4] = sentences.map(|s| {
            let words = s.split([' ','.']).cv();
            let mut req = [0; 4];
            words[4..].split(|&x| x == "and").for_each(|s| {
                req[rid[s[1]]] = s[0].parse::<i64>().unwrap();
            });
            req
        }).cv().try_into().unwrap();
        // blueprints.push(robots);
        let r = dijk(&robot_costs);
        eprintln!("{r} {id}");
        ans += r*id;
    }
    ans
}

fn dijk2(costs: &[[i64; 4]; 4]) -> i64 {
    let mut q = BinaryHeap::new();
    let mut v = HashSet::new();
    q.push((0, 1, [1,0,0,0], [0,0,0,0]));

    while let Some((c, t, b, r)) = q.pop() {
        if v.contains(&(b,r)) { continue; }
        v.insert((b, r));
        let mut r2 = r;
        for i in 0..4 {
            r2[i] += b[i];
        }
        if t == 32 {
            eprintln!("r2 = {r2:?}");
            eprintln!("b  = {b:?}");
            eprintln!("{c}");
            return r2[3];
        }
        for i in 0..4 {
            if sat(costs, &r, i) {
                let mut b = b;
                let mut r2 = r2;
                b[i] += 1;
                r2[0] -= costs[i][0];
                r2[1] -= costs[i][1];
                r2[2] -= costs[i][2];
                r2[3] -= costs[i][3];
                if !v.contains(&(b, r2)) {
                    if i == 3 {
                        q.push((c, t+1, b, r2));
                    } else {
                        q.push((c-(32-t), t+1, b, r2));
                    }
                }
            }
        }
        q.push((c-(32-t), t+1, b, r2));
    }
    -1
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut rid = HashMap::new();
    rid.insert("ore", 0);
    rid.insert("clay", 1);
    rid.insert("obsidian", 2);
    rid.insert("geode", 3);
    // let mut blueprints = vec![];
    let mut ans=1;
    for line in input.lines() {
        let (b, s) = line.split_once(": ").unwrap();
        let id = b.split_whitespace().cv()[1].parse::<i64>().unwrap();
        if id >3 { break; }
        let sentences = s.split(". ");
        let robot_costs: [[i64; 4]; 4] = sentences.map(|s| {
            let words = s.split([' ','.']).cv();
            let mut req = [0; 4];
            words[4..].split(|&x| x == "and").for_each(|s| {
                req[rid[s[1]]] = s[0].parse::<i64>().unwrap();
            });
            req
        }).cv().try_into().unwrap();
        // blueprints.push(robots);
        let r = dijk2(&robot_costs);
        eprintln!("{r} {id}");
        ans *= r;
    }
    ans
}
