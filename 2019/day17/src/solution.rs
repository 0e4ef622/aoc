use std::collections::*;
use std::borrow::Cow;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut vm = ic::Icvm::new(prog);
    vm.run();

    let mut map = vec![vec![]];
    for ch in vm.drain_outputs() {
        if ch == 10 {
            map.push(vec![]);
        } else {
            map.last_mut().unwrap().push(ch as u8);
        }
    }
    map.pop();
    map.pop();
    let mut sum = 0;
    for y in 1..map.len()-1 {
        for x in 1..map[0].len()-1 {
            if map[y][x] == b'#' &&
               map[y+1][x] == b'#' &&
               map[y-1][x] == b'#' &&
               map[y][x+1] == b'#' &&
               map[y][x-1] == b'#' {
                   sum += x*y;
            }
        }
    }
    sum
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Inst {
    L,
    R,
    F(usize),
    A,
    B,
    C,
}

impl Inst {
    fn is_fn(self) -> bool {
        use Inst::*;
        match self {
            L | R | F(_) => false,
            A | B | C => true,
        }
    }
}

fn serialize_run(run: &[Inst]) -> String {
    let mut result = String::new();
    let mut first = true;
    for inst in run {
        let s: Cow<'_, str> = match inst {
            Inst::L => "L".into(),
            Inst::R => "R".into(),
            Inst::A => "A".into(),
            Inst::B => "B".into(),
            Inst::C => "C".into(),
            Inst::F(n) => n.to_string().into(),
        };
        if first {
            first = false;
        } else {
            result.push(',');
        }
        result.push_str(&s);
    }
    result
}

fn get_run(map: &[Vec<u8>]) -> Vec<Inst> {
    let (mut x, mut y) = map
        .iter()
        .enumerate()
        .flat_map(|(y, v)|
            v.iter()
             .enumerate()
             .find(|&(_, &c)| c == b'^')
             .map(|(x, _)| (x, y)))
        .next().unwrap();

    let mut cd = 0; // 0 = up, 1 = right, 2 = down, 3 = left
    let mut insts = vec![];

    loop {
        let above = map.get((y as isize-1) as usize).and_then(|v| v.get(x)).unwrap_or(&b'.');
        let below = map.get(y+1).and_then(|v| v.get(x)).unwrap_or(&b'.');
        let left = map.get(y).and_then(|v| v.get((x as isize-1) as usize)).unwrap_or(&b'.');
        let right = map.get(y).and_then(|v| v.get(x+1)).unwrap_or(&b'.');

        match (cd, *above, *right, *below, *left) {
            (0, b'.', b'#', b'#', b'.') => { insts.push(Inst::R); cd = 1; }
            (0, b'.', b'#', b'.', b'.') => { insts.push(Inst::R); cd = 1; }
            (0, b'.', b'.', b'#', b'#') => { insts.push(Inst::L); cd = 3; }
            (0, b'.', b'.', b'.', b'#') => { insts.push(Inst::L); cd = 3; }

            (2, b'#', b'#', b'.', b'.') => { insts.push(Inst::L); cd = 1; }
            (2, b'.', b'#', b'.', b'.') => { insts.push(Inst::L); cd = 1; }
            (2, b'#', b'.', b'.', b'#') => { insts.push(Inst::R); cd = 3; }
            (2, b'.', b'.', b'.', b'#') => { insts.push(Inst::R); cd = 3; }

            (1, b'#', b'.', b'.', b'#') => { insts.push(Inst::L); cd = 0; }
            (1, b'#', b'.', b'.', b'.') => { insts.push(Inst::L); cd = 0; }
            (1, b'.', b'.', b'#', b'#') => { insts.push(Inst::R); cd = 2; }
            (1, b'.', b'.', b'#', b'.') => { insts.push(Inst::R); cd = 2; }
            (3, b'#', b'#', b'.', b'.') => { insts.push(Inst::R); cd = 0; }
            (3, b'#', b'.', b'.', b'.') => { insts.push(Inst::R); cd = 0; }
            (3, b'.', b'#', b'#', b'.') => { insts.push(Inst::L); cd = 2; }
            (3, b'.', b'.', b'#', b'.') => { insts.push(Inst::L); cd = 2; }
            c => panic!("bad {:?} at {} {}", c, x, y),
        }

        let mut count = 0;
        loop {

            let (nx, ny) = match cd {
                0 => (x, (y as isize-1) as usize),
                1 => (x+1, y),
                2 => (x, y+1),
                3 => ((x as isize-1) as usize, y),
                _ => panic!("ono"),
            };
            if map.get(ny).and_then(|v| v.get(nx)).unwrap_or(&b'.') == &b'#' {
                count += 1;
                x = nx;
                y = ny;
            } else {
                break;
            }
        }
        insts.push(Inst::F(count));

        let above = map.get((y as isize-1) as usize).and_then(|v| v.get(x)).unwrap_or(&b'.');
        let below = map.get(y+1).and_then(|v| v.get(x)).unwrap_or(&b'.');
        let left = map.get(y).and_then(|v| v.get((x as isize-1) as usize)).unwrap_or(&b'.');
        let right = map.get(y).and_then(|v| v.get(x+1)).unwrap_or(&b'.');

        match (cd, *above, *right, *below, *left) {
            (0, b'.', b'.', b'#', b'.')
            | (1, b'.', b'.', b'.', b'#')
            | (2, b'#', b'.', b'.', b'.')
            | (3, b'.', b'#', b'.', b'.') => break,
            _ => (),
        }
    }
    insts
}

fn find_fns(run: &[Inst], fns: Vec<Vec<Inst>>) -> Option<(Vec<Inst>, Vec<Vec<Inst>>)> {
    if run.iter().all(|i| i.is_fn()) && fns.len() != 3 { return None; }
    if fns.len() == 3 {
        if run.iter().all(|i| i.is_fn()) &&
            serialize_run(run).len() <= 20 &&
            fns.iter().all(|v| serialize_run(&v).len() <= 20) {
            return Some((run.to_vec(), fns));
        } else {
            return None;
        }
    }

    let mut iter = run.iter().enumerate().skip_while(|(_, i)| i.is_fn()).take_while(|(_, i)| !i.is_fn());
    let begin = iter.next().unwrap().0;
    let end = begin + iter.count() + 1;
    let fn_name = match fns.len() {
        0 => Inst::A,
        1 => Inst::B,
        2 => Inst::C,
        _ => panic!("baddddd"),
    };

    for i in begin+1..=end {
        let new_fn = run[begin..i].to_vec();
        let mut new_run = vec![];
        let mut run_slice = run;
        while !run_slice.is_empty() {
            if run_slice.len() >= new_fn.len() && run_slice[..new_fn.len()] == new_fn[..] {
                new_run.push(fn_name);
                run_slice = &run_slice[new_fn.len()..];
            } else {
                new_run.push(run_slice[0]);
                run_slice = &run_slice[1..];
            }
        }
        let mut new_fns = fns.clone();
        new_fns.push(new_fn);

        if let Some((a, b)) = find_fns(&new_run, new_fns) {
            return Some((a, b));
        }
    }
    return None;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut vm = ic::Icvm::new(prog);
    vm.run();

    let mut map = vec![vec![]];
    for ch in vm.drain_outputs() {
        if ch == 10 {
            map.push(vec![]);
        } else {
            map.last_mut().unwrap().push(ch as u8);
        }
    }
    map.pop();
    map.pop();
    let run = get_run(&map);
    let (main, fns) = find_fns(&run, vec![]).unwrap();

    let main = serialize_run(&main);
    let a = serialize_run(&fns[0]);
    let b = serialize_run(&fns[1]);
    let c = serialize_run(&fns[2]);
    println!("main: {}", main);
    println!("A: {}", a);
    println!("B: {}", b);
    println!("C: {}", c);

    let mut prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    prog[0] = 2;

    vm = ic::Icvm::new(prog);

    vm.push_inputs(main.bytes());
    vm.push_input(10);
    vm.push_inputs(a.bytes());
    vm.push_input(10);
    vm.push_inputs(b.bytes());
    vm.push_input(10);
    vm.push_inputs(c.bytes());
    vm.push_input(10);
    vm.push_input(b'n');
    vm.push_input(10);

    vm.run();
    let x = vm.drain_outputs().last().unwrap();
    x
}
