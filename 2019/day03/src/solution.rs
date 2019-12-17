use std::collections::HashSet;
use std::collections::HashMap;
pub fn part1(input: &str) -> isize {
    let w = input.lines().map(|l| l.split(",").map(|s| (&s[0..1], s[1..].parse::<isize>().unwrap())).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut closest = (999999999999999isize, 999999999999999999isize);
    let mut visited = HashSet::new();
    for (i, wire) in w.into_iter().enumerate() {
        let mut p = (0isize, 0isize);
        for (dir, dist) in wire {
            for _ in 0..dist {
                match dir {
                    "U" => p.1 += 1,
                    "D" => p.1 -= 1,
                    "L" => p.0 -= 1,
                    "R" => p.0 += 1,
                    _ => (),
                }
                if visited.contains(&p) && i == 1 && p.0.abs()+p.1.abs() < closest.0.abs()+closest.1.abs() {
                    closest = p;
                }
                if i == 0 { visited.insert(p); }
            }
        }
    }
    closest.0.abs()+closest.1.abs()
}

pub fn part2(input: &str) -> isize {
    let w = input.lines().map(|l| l.split(",").map(|s| (&s[0..1], s[1..].parse::<isize>().unwrap())).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut closest = (999999999999999isize, 999999999999999999isize);
    let mut steps = 999999999999isize;
    let mut visited = HashMap::new();
    for (i, wire) in w.into_iter().enumerate() {
        let mut p = (0isize, 0isize);
        let mut s = 0isize;
        for (dir, dist) in wire {
            for _ in 0..dist {
                s += 1;
                match dir {
                    "U" => p.1 += 1,
                    "D" => p.1 -= 1,
                    "L" => p.0 -= 1,
                    "R" => p.0 += 1,
                    _ => (),
                }
                if visited.get(&p).map(|x| s+x < steps) == Some(true) && i == 1 {
                    closest = p;
                    steps = visited[&p] + s;
                }
                if i == 0 { visited.insert(p, s); }
            }
        }
    }
    steps
}
