use std::{cmp::Reverse, collections::*};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

fn mv(p: &mut [P<i64>], action: u8, numpad: &Grid<u8>, dpad: &Grid<u8>) -> Option<Option<u8>> {
    let mut last = *p.last().unwrap();
    match action {
        b'<' => last = last + Dir::L,
        b'^' => last = last + Dir::U,
        b'>' => last = last + Dir::R,
        b'v' => last = last + Dir::D,
        b'A' => if p.len() > 1 {
            let len = p.len();
            return mv(&mut p[..len-1], dpad[last], numpad, dpad);
        } else {
            return Some(Some(numpad[last]));
        }
        _ => unreachable!(),
    }
    let pad = if p.len() == 1 {
        numpad
    } else {
        dpad
    };
    if matches!(pad.get(last), None | Some(&b' ')) {
        None
    } else {
        *p.last_mut().unwrap() = last;
        Some(None)
    }
}

fn calc(code: &str, numpad: &Grid<u8>, dpad: &Grid<u8>) -> usize {
    let numpad_a = numpad.find(b'A').unwrap();
    let dpad_a = dpad.find(b'A').unwrap();

    let mut q = VecDeque::new();
    let mut dist = HashMap::new();
    q.push_back(Reverse((0, numpad_a, dpad_a, dpad_a, 0)));
    while let Some(Reverse((d, np, dp1, dp2, i))) = q.pop_front() {
        if dist.contains_key(&(np, dp1, dp2, i)) {
            continue;
        }
        if i == code.len() {
            return d;
        }
        dist.insert((np, dp1, dp2, i), d);
        for &a in b"<^v>A" {
            let mut s = [np, dp1, dp2];
            match mv(&mut s, a, numpad, dpad) {
                Some(None) => {
                    q.push_back(Reverse((d+1, s[0], s[1], s[2], i)));
                }
                Some(Some(ch)) if ch == code.as_bytes()[i] => {
                    q.push_back(Reverse((d+1, s[0], s[1], s[2], i+1)));
                }
                _ => (),
            }
        }
    }
    unreachable!();
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let numpad = Grid::from_ascii("789\n456\n123\n 0A");
    let dpad = Grid::from_ascii(" ^A\n<v>");

    let mut ans = 0;
    for code in input.lines() {
        let len = calc(code, &numpad, &dpad);
        ans += len * code[..3].parse::<usize>().unwrap();
    }

    ans
}

fn calc2(code: &str, numpad: &Grid<u8>, dpad: &Grid<u8>, dp: &[[[usize; 5]; 5]; 26], depth: usize) -> usize {
    let numpad_a = numpad.find(b'A').unwrap();
    let dpad_a = dpad.find(b'A').unwrap();
    let mut q = BinaryHeap::new();
    let mut dist = HashMap::new();
    q.push(Reverse((0, [numpad_a], 4, 0)));
    while let Some(Reverse((cdist, ss, pa, i))) = q.pop() {
        if dist.contains_key(&(ss, pa, i)) {
            continue;
        }
        if i == code.len() {
            return cdist;
        }
        dist.insert((ss, pa, i), cdist);
        for (j, &a) in b"<^v>A".iter().enumerate() {
            let mut s = ss;
            match mv(&mut s, a, numpad, dpad) {
                Some(None) => {
                    q.push(Reverse((cdist + dp[depth][pa][j], s, j, i)));
                }
                Some(Some(ch)) if ch == code.as_bytes()[i] => {
                    q.push(Reverse((cdist + dp[depth][pa][j], s, j, i+1)));
                }
                _ => (),
            }
        }
    }
    unreachable!();
}

fn build_table(dp: &mut [[[usize; 5]; 5]; 26], l: usize, dpad: &Grid<u8>) {
    for (ai, &ac) in b"<^v>A".iter().enumerate() {
        let start = dpad.find(ac).unwrap();

        let mut q = BinaryHeap::new();
        let mut dist = HashMap::new();
        q.push(Reverse((0, start, 4)));
        while let Some(Reverse((d, p, pa))) = q.pop() {
            if dist.contains_key(&(p, pa)) {
                continue;
            }
            dist.insert((p, pa), d);
            for (j, &a) in b"<^v>A".iter().enumerate() {
                let mut s = [p];
                match mv(&mut s, a, dpad, dpad) {
                    Some(None) | Some(Some(_)) => {
                        q.push(Reverse((d + dp[l-1][pa][j], s[0], j)));
                    }
                    _ => (),
                }
            }
        }

        for (j, &a) in b"<^v>A".iter().enumerate() {
            dp[l][ai][j] = dist[&(dpad.find(a).unwrap(), 4)];
        }
        dp[l][ai][ai] = 1;
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let numpad = Grid::from_ascii("789\n456\n123\n 0A");
    let dpad = Grid::from_ascii(" ^A\n<v>");

    // < ^ v > A
    let mut dp = [[[0; 5]; 5]; 26];
    // dp[level][action1][action2] = cost to move from <action1> to <action2> and execute
    dp[0] = [[1; 5]; 5];
    for l in 1..dp.len() {
        build_table(&mut dp, l, &dpad);
    }

    let mut ans = 0;
    for code in input.lines() {
        let len = calc2(code, &numpad, &dpad, &dp, 25);
        ans += len * code[..3].parse::<usize>().unwrap();
    }

    assert_ne!(ans, 724821702235479);
    assert_ne!(ans, 730831226684953758);
    ans
}
