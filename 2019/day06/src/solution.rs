use std::collections::*;
pub fn part1(input: &str) -> usize {

    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut s = line.split(")");
        let c = s.next().unwrap();
        let o = s.next().unwrap();

        graph.entry(c).or_insert(vec![]).push(o);
    }

    rec(&graph, "COM", 0)
}

fn rec<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, n: &'a str, l: usize) -> usize {
    l + graph.get(n).map(|v| v.iter().map(|n| rec(graph, n, l+1)).sum()).unwrap_or(0)
}

pub fn part2(input: &str) -> usize {

    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut s = line.split(")");
        let c = s.next().unwrap();
        let o = s.next().unwrap();

        graph.insert(o, c);
    }

    let mut orbits = HashMap::new();

    let mut p = "YOU";
    let mut i = 0;
    while p != "COM" {
        p = graph[p];
        i += 1;
        orbits.insert(p, i);
    }

    let mut p = "SAN";
    let mut j = 0;
    while !orbits.contains_key(p) {
        p = graph[p];
        j += 1;
    }

    orbits[p] + j - 2
}
