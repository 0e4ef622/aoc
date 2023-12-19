use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn xmas(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Vec<Vec<i64>>, HashMap<&str, Vec<(usize, &str, i64, &str)>>) {
    let sec = input.split("\n\n").cv();
    let parts = sec[1].lines().map(|s| s[1..s.len()-1].split(',').map(|ss| ss[2..].parse::<i64>().unwrap()).cv()).cv();
    let rules: HashMap<_, _> = sec[0].lines().map(|l| {
        let b = l.find('{').unwrap();
        let name = &l[..b];
        let branches = l[b+1..l.len()-1].split(',').map(|x| {
            if let Some(c) = x.find(':') {
                (xmas(&x[..1]), &x[1..2], x[2..c].parse::<i64>().unwrap(), &x[c+1..])
            } else {
                (5, "", 0, x)
            }
        }).cv();
        (name, branches)
    }).collect();
    (parts, rules)
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (parts, rules) = parse(input);

    let run = |x: [i64; 4]| {
        let mut rule = "in";
        while rule != "A" && rule != "R" {
            for (i, op, thr, ne) in &rules[rule] {
                if *i == 5 {
                    rule = ne;
                } else {
                    match *op {
                        "<" if x[*i] < *thr => { rule = ne; break; }
                        ">" if x[*i] > *thr => { rule = ne; break; }
                        _ => (),
                    }
                }
            }
        }
        rule == "A"
    };
    let mut ans = 0;
    for p in parts {
        let pp = [p[0], p[1], p[2], p[3]];
        if run(pp) {
            ans += p[0] + p[1] + p[2] + p[3];
        }
    }
    ans
}

fn dfs(rules: &HashMap<&str, Vec<(usize, &str, i64, &str)>>, mut lb: [i64; 4], mut ub: [i64; 4], rule: &str) -> i64 {
    if rule == "A" {
        let mut ans = 1;
        for i in 0..4 {
            ans *= (ub[i]-lb[i]).max(0);
        }
        return ans
    }
    if rule == "R" {
        return 0;
    }

    let mut ans = 0;
    for (i, op, thr, ne) in &rules[rule] {
        let i = *i;
        if i == 5 {
            ans += dfs(rules, lb, ub, ne);
        } else {
            match *op {
                "<" => {
                    let mut ub2 = ub;
                    ub2[i] = ub2[i].min(*thr);
                    ans += dfs(rules, lb, ub2, ne);
                    lb[i] = lb[i].max(*thr);
                }
                ">" => {
                    let mut lb2 = lb;
                    lb2[i] = lb2[i].max(*thr+1);
                    ans += dfs(rules, lb2, ub, ne);
                    ub[i] = ub[i].min(*thr+1);
                }
                _ => unreachable!(),
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (_parts, rules) = parse(input);
    dfs(&rules, [1; 4], [4001; 4], "in")
}
