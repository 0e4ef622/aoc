use std::collections::*;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g: HashMap<String, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let s: Vec<_> = line.trim().splitn(2, " bags contain ").collect();
        let outer = s[0];
        let inners = s[1].trim_matches('.').split(", ").map(|e| {
            let s = e.split_whitespace().collect::<Vec<_>>();
            s[1].to_string() + " " + s[2]
        });
        g.entry(outer.into()).or_default();
        for inner in inners {
            g.entry(inner).or_default().push(outer);
        }
    }

    let mut v = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back("shiny gold");
    while !q.is_empty() {
        let e = q.pop_front().unwrap();
        if v.contains(&e) { continue; }
        for x in &g[&*e] {
            q.push_back(x);
        }
        v.insert(e);
    }
    v.len() - 1
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in input.lines() {
        let s: Vec<_> = line.trim().splitn(2, " bags contain ").collect();
        let outer = s[0];
        let inners = s[1].trim_matches('.').split(", ").map(|e| {
            let s = e.split_whitespace().collect::<Vec<_>>();
            (s[0].trim().parse().unwrap_or(0), s[1].to_string() + " " + s[2])
        });
        for inner in inners {
            g.entry(inner.1.clone()).or_default();
            g.entry(outer.into()).or_default().push(inner);
        }
    }

    let mut q = VecDeque::new();
    let mut c = 0;
    q.push_back((1, "shiny gold".to_string()));
    while !q.is_empty() {
        let (a, e) = q.pop_front().unwrap();
        for x in &g[&*e] {
            q.push_back((a*x.0, x.1.clone()));
        }
        c += a;
    }
    c - 1
}
