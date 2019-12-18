use std::collections::*;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut map: Vec<Vec<u8>> = input.trim().lines().map(|l| l.bytes().collect()).collect();

    let pos = map.iter().enumerate().flat_map(|(y, v)| Some((v.iter().enumerate().find(|(_,c)| **c == b'@')?.0, y))).next().unwrap();

    let mut keys = HashMap::new();
    for c in b'a'..=b'z' {
        let mut kpos = map.iter().enumerate().flat_map(|(y, v)| Some((v.iter().enumerate().find(|(_,x)| **x == c)?.0, y))).next();
        if let Some(k) = kpos {
            keys.insert(c, k);
            map[k.1][k.0] = b'.';
        }
    }

    let mut adj_map = HashMap::new();

    for &pos1 in keys.values() {
        let dist_rq = path_len(pos, pos1, &map).unwrap();
        adj_map.insert((pos, pos1), dist_rq.clone());
        adj_map.insert((pos1, pos), dist_rq);

        for &pos2 in keys.values() {
            let dist_rq = path_len(pos1, pos2, &map).unwrap();
            adj_map.insert((pos2, pos1), dist_rq.clone());
            adj_map.insert((pos1, pos2), dist_rq);
        }
    }

    use std::cmp::Reverse;

    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();

    q.push((Reverse(0), pos, BTreeSet::new()));

    loop {
        let mut head = if let Some(x) = q.pop() { x } else { panic!("bfs fail") };

        if visited.contains(&(head.1, head.2.clone())) { continue; }

        if head.2.len() == keys.len() { break head.0 .0; }

        visited.insert((head.1, head.2.clone()));

        for (&key, &kpos) in &keys {
            if head.2.contains(&key) { continue; }
            let (dist, rqk) = &adj_map[&(head.1, kpos)];
            if !rqk.iter().all(|k| head.2.contains(k)) {
                continue; // skip it
            } else {
                let mut newset = head.2.clone();
                newset.insert(key);
                q.push((Reverse(head.0 .0+dist), kpos, newset));
            }
        }
    }
}

fn path_len(from: (usize, usize), to: (usize, usize), map: &[Vec<u8>]) -> Option<(usize, Vec<u8>)> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((0, from, vec![]));

    loop {
        let mut head = if let Some(x) = q.pop_front() { x } else { return None; };

        if visited.contains(&head.1) { continue; }

        if head.1 == to {
            break Some((head.0, head.2));
        }

        visited.insert(head.1);

        match map[head.1 .1][head.1 .0] {
            b'#' => continue,
            door @ b'A'..=b'Z' => head.2.push(door - b'A' + b'a'),
            _ => (),
        }

        q.push_back((head.0 + 1, (head.1 .0 + 1, head.1 .1), head.2.clone()));
        q.push_back((head.0 + 1, (head.1 .0 - 1, head.1 .1), head.2.clone()));
        q.push_back((head.0 + 1, (head.1 .0, head.1 .1 + 1), head.2.clone()));
        q.push_back((head.0 + 1, (head.1 .0, head.1 .1 - 1), head.2));
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut map: Vec<Vec<u8>> = input.trim().lines().map(|l| l.bytes().collect()).collect();

    let pos = map.iter().enumerate().flat_map(|(y, v)| Some((v.iter().enumerate().find(|(_,c)| **c == b'@')?.0, y))).next().unwrap();
    map[pos.1+1][pos.0] = b'#';
    map[pos.1-1][pos.0] = b'#';
    map[pos.1][pos.0+1] = b'#';
    map[pos.1][pos.0-1] = b'#';

    let robot_pos = [
        (pos.0+1, pos.1+1),
        (pos.0+1, pos.1-1),
        (pos.0-1, pos.1+1),
        (pos.0-1, pos.1-1),
    ];

    let mut keys = HashMap::new();
    for c in b'a'..=b'z' {
        let mut kpos = map.iter().enumerate().flat_map(|(y, v)| Some((v.iter().enumerate().find(|(_,x)| **x == c)?.0, y))).next();
        if let Some(k) = kpos {
            keys.insert(c, k);
            map[k.1][k.0] = b'.';
        }
    }

    let mut adj_maps = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];

    for &pos1 in keys.values() {

        for (&robot, adj_map) in robot_pos.iter().zip(&mut adj_maps) {
            let dist_rq = match path_len(robot, pos1, &map) {
                Some(x) => x,
                None => continue,
            };
            adj_map.insert((robot, pos1), dist_rq.clone());
            adj_map.insert((pos1, robot), dist_rq);
        }

        for &pos2 in keys.values() {
            let dist_rq = match path_len(pos1, pos2, &map) {
                Some(x) => x,
                None => continue,
            };
            let mut adj_map = adj_maps.iter_mut().zip(&robot_pos).find(|(m, p)| m.contains_key(&(**p, pos1))).unwrap().0;
            adj_map.insert((pos2, pos1), dist_rq.clone());
            adj_map.insert((pos1, pos2), dist_rq);
        }
    }

    use std::cmp::Reverse;

    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();

    q.push((Reverse(0), robot_pos, BTreeSet::new()));

    loop {
        let mut head = if let Some(x) = q.pop() { x } else { panic!("bfs fail") };

        if visited.contains(&(head.1, head.2.clone())) { continue; }

        if head.2.len() == keys.len() { break head.0 .0; }

        visited.insert((head.1, head.2.clone()));

        for (&key, &kpos) in &keys {

            if head.2.contains(&key) { continue; }

            let (robot, (dist, rqk)) = adj_maps.iter().zip(&head.1).enumerate().find_map(|(i, (m, r))| m.get(&(*r, kpos)).map(|v| (i,v))).expect("wat");

            if !rqk.iter().all(|k| head.2.contains(k)) {
                continue; // skip it
            } else {
                let mut newset = head.2.clone();
                newset.insert(key);
                let mut newbots = head.1;
                newbots[robot] = kpos;
                q.push((Reverse(head.0 .0+dist), newbots, newset));
            }
        }
    }
}
