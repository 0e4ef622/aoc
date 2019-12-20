use std::collections::*;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let mut a = 'a: loop {
        for y in 2..map.len()-2 {
            for x in 2..map[0].len()-2 {
                if is_portal(&map, [x, y]) == Some([b'A', b'A']) { break 'a [x, y]; }
            }
        }
    };

    let mut z = 'b: loop {
        for y in 2..map.len()-2 {
            for x in 2..map[0].len()-2 {
                if is_portal(&map, [x, y]) == Some([b'Z', b'Z']) { break 'b [x, y]; }
            }
        }
    };

    use std::cmp::Reverse;

    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push((Reverse(0), a));

    loop {
        let head = q.pop().expect("fail");

        if visited.contains(&head.1) { continue; }
        if head.1 == z {
            return head.0 .0;
        }
        visited.insert(head.1);

        let [x,y] = head.1;

        if map[y-1][x] == b'.' { q.push((Reverse(head.0 .0 + 1), [x, y-1])); }
        if map[y+1][x] == b'.' { q.push((Reverse(head.0 .0 + 1), [x, y+1])); }
        if map[y][x-1] == b'.' { q.push((Reverse(head.0 .0 + 1), [x-1, y])); }
        if map[y][x+1] == b'.' { q.push((Reverse(head.0 .0 + 1), [x+1, y])); }
        let portal = is_portal(&map, [x, y]);
        if let Some(p) = portal {
            if p != [b'A', b'A'] && p != [b'Z', b'Z'] {
                q.push((Reverse(head.0 .0 + 1), warp(&map, p, [x, y])));
            }
        }
    }
}

fn is_portal(map: &[Vec<u8>], [x, y]: [usize; 2]) -> Option<[u8; 2]> {
    if map[y][x] == b'.' {
        match [map[y-1][x], map[y-2][x]] {
            x @ [b'A'..=b'Z', b'A'..=b'Z'] => return Some(x),
            _ => (),
        }
        match [map[y+1][x], map[y+2][x]] {
            x @ [b'A'..=b'Z', b'A'..=b'Z'] => return Some(x),
            _ => (),
        }
        match [map[y][x-1], map[y][x-2]] {
            x @ [b'A'..=b'Z', b'A'..=b'Z'] => return Some(x),
            _ => (),
        }
        match [map[y][x+1], map[y][x+2]] {
            x @ [b'A'..=b'Z', b'A'..=b'Z'] => return Some(x),
            _ => (),
        }
    }
    None
}

fn is_inner(map: &[Vec<u8>], [x, y]: [usize; 2]) -> bool {
    x > 2 && x < map[0].len() - 3 &&
        y > 2 && y < map.len() - 3
}

fn warp(map: &[Vec<u8>], portal: [u8; 2], p: [usize; 2]) -> [usize; 2] {
    for y in 2..map.len()-2 {
        for x in 2..map[0].len()-2 {
            if p == [x, y] { continue; }
            let r = is_portal(&map, [x, y]);
            if r == Some(portal) || r == Some([portal[1], portal[0]]) {
                return [x, y];
            }
        }
    }
    panic!("warp failed")
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let mut a = 'a: loop {
        for y in 2..map.len()-2 {
            for x in 2..map[0].len()-2 {
                if is_portal(&map, [x, y]) == Some([b'A', b'A']) { break 'a [x, y]; }
            }
        }
    };

    let mut z = 'b: loop {
        for y in 2..map.len()-2 {
            for x in 2..map[0].len()-2 {
                if is_portal(&map, [x, y]) == Some([b'Z', b'Z']) { break 'b [x, y]; }
            }
        }
    };

    use std::cmp::Reverse;

    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push((Reverse(0), a, 0));

    loop {
        let head = q.pop().expect("fail");

        if visited.contains(&(head.1, head.2)) { continue; }
        if head.1 == z && head.2 == 0 {
            return head.0 .0;
        }
        visited.insert((head.1, head.2));

        let [x,y] = head.1;

        if map[y-1][x] == b'.' && !visited.contains(&([x, y-1], head.2)) { q.push((Reverse(head.0 .0 + 1), [x, y-1], head.2)); }
        if map[y+1][x] == b'.' && !visited.contains(&([x, y+1], head.2)) { q.push((Reverse(head.0 .0 + 1), [x, y+1], head.2)); }
        if map[y][x-1] == b'.' && !visited.contains(&([x-1, y], head.2)) { q.push((Reverse(head.0 .0 + 1), [x-1, y], head.2)); }
        if map[y][x+1] == b'.' && !visited.contains(&([x+1, y], head.2)) { q.push((Reverse(head.0 .0 + 1), [x+1, y], head.2)); }

        let portal = is_portal(&map, [x, y]);
        if let Some(p) = portal {
            if p != [b'A', b'A'] && p != [b'Z', b'Z'] {
                if is_inner(&map, [x, y]) {
                    q.push((Reverse(head.0 .0 + 1), warp(&map, p, [x, y]), head.2 + 1));
                } else if head.2 > 0 {
                    q.push((Reverse(head.0 .0 + 1), warp(&map, p, [x, y]), head.2 - 1));
                }
            }
        }
    }
}
