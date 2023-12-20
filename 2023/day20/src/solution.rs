use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut mods = HashMap::new();
    let mut ff = HashMap::new();
    let mut conj = HashMap::new();
    for line in input.lines() {
        let w = line.split(" -> ").cv();
        let dest = w[1].split(", ").cv();
        if w[0] == "broadcaster" {
            mods.insert("brd", ("brd", dest));
        } else {
            mods.insert(&w[0][1..], (&w[0][..1], dest));
            if &w[0][..1] == "%" {
                ff.insert(&w[0][1..], false);
            } else {
                conj.insert(&w[0][1..], HashMap::new());
            }
        }
    }

    for (name, (_, out)) in &mods {
        for outt in out {
            if mods.get(outt).map(|x| x.0) == Some("&") {
                conj.get_mut(outt).unwrap().insert(name, false);
            }
        }
    }

    let mut hi_cnt = 0i64;
    let mut lo_cnt = 0i64;
    let mut q = VecDeque::new();
    for _ in 0..1000 {
        lo_cnt += 1;
        for x in &mods["brd"].1 {
            q.push_back((x, false, "brd"));
            lo_cnt += 1;
        }
        while let Some((name, value, from)) = q.pop_front() {
            // eprintln!("{name} received {value} from {from}");
            if !mods.contains_key(name) { continue; }
            match mods[name].0 {
                "%" if value == false => {
                    *ff.get_mut(name).unwrap() = !ff[name];
                    let v = ff[name];
                    for out in &mods[name].1 {
                        q.push_back((out, v, name));
                        if v { hi_cnt += 1; }
                        else { lo_cnt += 1; }
                    }
                }
                "&" => {
                    let state = conj.get_mut(name).unwrap();
                    *state.get_mut(&from).unwrap() = value;
                    let v = !state.values().all(|x| *x);
                    for out in &mods[name].1 {
                        q.push_back((out, v, name));
                        if v { hi_cnt += 1; }
                        else { lo_cnt += 1; }
                    }
                }
                _ => (),
            }
        }
    }
    hi_cnt*lo_cnt
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut mods = HashMap::new();
    let mut rmods = HashMap::new();
    let mut ff = HashMap::new();
    let mut conj = HashMap::new();
    for line in input.lines() {
        let w = line.split(" -> ").cv();
        let dest = w[1].split(", ").cv();
        let mut nn = "brd";
        if w[0] == "broadcaster" {
            mods.insert("brd", ("brd", dest.clone()));
        } else {
            nn = &w[0][1..];
            mods.insert(nn, (&w[0][..1], dest.clone()));
            if &w[0][..1] == "%" {
                ff.insert(&w[0][1..], false);
            } else {
                conj.insert(&w[0][1..], HashMap::new());
            }
        }
        for d in dest {
            rmods.entry(d).or_insert(vec![]).push(nn);
        }
    }

    let mut important = HashMap::new();
    let mut bits = vec![];
    for (name, inputs) in &rmods {
        if *name == "rx" {
            for input in &rmods[inputs[0]] {
                let i = bits.len();
                bits.push(0);
                for ii in &rmods[rmods[input][0]] {
                    important.insert(ii, i);
                }
            }
        } else if mods.get(name).map(|x| x.0) == Some("&") {
            for input in inputs {
                conj.get_mut(name).unwrap().insert(input, false);
            }
        }
    }

    let mut hi_cnt = 0i64;
    let mut lo_cnt = 0i64;
    let mut q = VecDeque::new();
    for bp in 1.. {
        if important.len() == 0 { break; }
        lo_cnt += 1;
        for x in &mods["brd"].1 {
            q.push_back((x, false, "brd"));
            lo_cnt += 1;
        }
        while let Some((name, value, from)) = q.pop_front() {
            // eprintln!("{name} received {value} from {from}");
            if important.contains_key(&from) && value {
                bits[important[&from]] += bp;
                important.remove(&from);
            }
            if !mods.contains_key(name) { continue; }
            match mods[name].0 {
                "%" if value == false => {
                    *ff.get_mut(name).unwrap() = !ff[name];
                    let v = ff[name];
                    for out in &mods[name].1 {
                        q.push_back((out, v, name));
                        if v { hi_cnt += 1; }
                        else { lo_cnt += 1; }
                    }
                }
                "&" => {
                    let state = conj.get_mut(name).unwrap();
                    *state.get_mut(&from).unwrap() = value;
                    let v = !state.values().all(|x| *x);
                    for out in &mods[name].1 {
                        q.push_back((out, v, name));
                        if v { hi_cnt += 1; }
                        else { lo_cnt += 1; }
                    }
                }
                _ => (),
            }
        }
    }
    bits.into_iter().reduce(lcm).unwrap()
}
