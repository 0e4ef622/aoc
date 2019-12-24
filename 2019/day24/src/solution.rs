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

    for _ in 0..200 {
        // for thing in &state {
        //     print_thing(thing);
        //     println!("-");
        // }
        // println!("-------------------------------------");
        state = next_state2(state);
    }
    state.into_iter().flatten().flatten().filter(|&x| x == b'#').count()
}

fn print_thing(thing: &[Vec<u8>]) {
    for line in thing {
        for &c in line {
            print!("{}", c as char);
        }
        println!();
    }
}

fn next_state2(mut state: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    state.insert(0, vec![vec![b'.'; 5]; 5]);
    state.push(vec![vec![b'.'; 5]; 5]);
    let state = state;
    let mut newstate = state.clone();

    for d in 0..state.len() {
        for y in 0..state[d].len() {
            for x in 0..state[d][y].len() {
                if x == 2 && y == 2 { continue; }
                let adji = adjacents([x, y], d);


                let mut adjc = 0;
                for ([X, Y], D) in adji {
                    if D >= state.len() { continue; }
                    if state[D][Y][X] == b'#' {
                        adjc += 1;
                    }
                }

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
    newstate
}

fn adjacents([x, y]: [usize; 2], d: usize) -> Vec<([usize; 2], usize)> {
    let mut adj = vec![];

    if x == 0 && d != 0 {
        adj.push(([1, 2], d-1));
    }

    if y == 0 && d != 0 {
        adj.push(([2, 1], d-1));
    }

    if x == 4 && d != 0 {
        adj.push(([3, 2], d-1));
    }
    if y == 4 && d != 0 {
        adj.push(([2, 3], d-1));
    }

    if x != 0 && !(x == 3 && y == 2) {
        adj.push(([x-1, y], d));
    }
    if y != 0 && !(y == 3 && x == 2) {
        adj.push(([x, y-1], d));
    }

    if x != 4 && !(x == 1 && y == 2) {
        adj.push(([x+1, y], d));
    }
    if y != 4 && !(y == 1 && x == 2) {
        adj.push(([x, y+1], d));
    }

    if y == 1 && x == 2 {
        adj.push(([0, 0], d+1));
        adj.push(([1, 0], d+1));
        adj.push(([2, 0], d+1));
        adj.push(([3, 0], d+1));
        adj.push(([4, 0], d+1));
    }
    if y == 2 && x == 1 {
        adj.push(([0, 0], d+1));
        adj.push(([0, 1], d+1));
        adj.push(([0, 2], d+1));
        adj.push(([0, 3], d+1));
        adj.push(([0, 4], d+1));
    }
    if y == 2 && x == 3 {
        adj.push(([4, 0], d+1));
        adj.push(([4, 1], d+1));
        adj.push(([4, 2], d+1));
        adj.push(([4, 3], d+1));
        adj.push(([4, 4], d+1));
    }
    if y == 3 && x == 2 {
        adj.push(([0, 4], d+1));
        adj.push(([1, 4], d+1));
        adj.push(([2, 4], d+1));
        adj.push(([3, 4], d+1));
        adj.push(([4, 4], d+1));
    }
    adj
}
