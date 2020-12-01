use std::collections::*;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut state: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let mut seen = HashSet::new();
    seen.insert(state.clone());
    loop {
        state = next_state(&state);
        if seen.contains(&state) {
            break;
        }
        seen.insert(state.clone());
    }
    biodiv(&state)
}

fn biodiv(state: &[Vec<u8>]) -> usize {
    let mut x = 0;
    let mut sum = 0;
    for y in 0..state.len() {
        for v in 0..state[y].len() {
            if state[y][v] == b'#' {
                sum += 2usize.pow(x as u32);
            }
            x += 1;
        }
    }
    sum
}
fn next_state(state: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut newstate = vec![vec![b'.'; state[0].len()]; state.len()];
    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let a = (*state.get((y as isize-1) as usize).and_then(|v| v.get(x)).unwrap_or(&b'.') == b'#') as i32;
            let b = (*state.get(y+1).and_then(|v| v.get(x)).unwrap_or(&b'.') == b'#') as i32;
            let c = (*state.get(y).and_then(|v| v.get(x+1)).unwrap_or(&b'.') == b'#') as i32;
            let d = (*state.get(y).and_then(|v| v.get((x as isize-1) as usize)).unwrap_or(&b'.') == b'#') as i32;

            if state[y][x] == b'#' && a+b+c+d != 1 {
                newstate[y][x] = b'.';
            } else if state[y][x] == b'.' && (a+b+c+d == 1 || a+b+c+d == 2) {
                newstate[y][x] = b'#';
            } else {
                newstate[y][x] = state[y][x];
            }
        }
    }
    newstate
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut state: Vec<Vec<Vec<u8>>> = vec![input.lines().map(|l| l.bytes().collect()).collect()];
    let mut oldstate: Vec<Vec<Vec<u8>>> = state.clone();

    for _ in 0..200 {
        next_state2(&mut oldstate, &mut state);
        let t = oldstate;
        oldstate = state;
        state = t;
    }
    oldstate.into_iter().flatten().flatten().filter(|&x| x == b'#').count()
}

fn next_state2(state: &mut Vec<Vec<Vec<u8>>>, newstate: &mut Vec<Vec<Vec<u8>>>) {
    state.insert(0, vec![vec![b'.'; 5]; 5]);
    state.push(vec![vec![b'.'; 5]; 5]);
    newstate.insert(0, vec![vec![b'.'; 5]; 5]);
    newstate.push(vec![vec![b'.'; 5]; 5]);

    for d in 0..state.len() {
        for y in 0..state[d].len() {
            for x in 0..state[d][y].len() {

                if x == 2 && y == 2 { continue; }
                let mut adjc = 0;
                adjacents([x, y], d, |[X, Y], D| {
                    if D >= state.len() { return; }
                    if state[D][Y][X] == b'#' {
                        adjc += 1;
                    }
                });

                if state[d][y][x] == b'#' && adjc != 1 {
                    newstate[d][y][x] = b'.';
                } else if state[d][y][x] == b'.' && (adjc == 1 || adjc == 2) {
                    newstate[d][y][x] = b'#';
                } else {
                    newstate[d][y][x] = state[d][y][x];
                }
            }
        }
    }
}

fn adjacents([x, y]: [usize; 2], d: usize, mut f: impl FnMut([usize; 2], usize)) {
    if x == 0 && d != 0 {
        f([1, 2], d-1);
    }

    if y == 0 && d != 0 {
        f([2, 1], d-1);
    }

    if x == 4 && d != 0 {
        f([3, 2], d-1);
    }
    if y == 4 && d != 0 {
        f([2, 3], d-1);
    }

    if x != 0 && !(x == 3 && y == 2) {
        f([x-1, y], d);
    }
    if y != 0 && !(y == 3 && x == 2) {
        f([x, y-1], d);
    }

    if x != 4 && !(x == 1 && y == 2) {
        f([x+1, y], d);
    }
    if y != 4 && !(y == 1 && x == 2) {
        f([x, y+1], d);
    }

    if y == 1 && x == 2 {
        f([0, 0], d+1);
        f([1, 0], d+1);
        f([2, 0], d+1);
        f([3, 0], d+1);
        f([4, 0], d+1);
    }
    if y == 2 && x == 1 {
        f([0, 0], d+1);
        f([0, 1], d+1);
        f([0, 2], d+1);
        f([0, 3], d+1);
        f([0, 4], d+1);
    }
    if y == 2 && x == 3 {
        f([4, 0], d+1);
        f([4, 1], d+1);
        f([4, 2], d+1);
        f([4, 3], d+1);
        f([4, 4], d+1);
    }
    if y == 3 && x == 2 {
        f([0, 4], d+1);
        f([1, 4], d+1);
        f([2, 4], d+1);
        f([3, 4], d+1);
        f([4, 4], d+1);
    }
}
