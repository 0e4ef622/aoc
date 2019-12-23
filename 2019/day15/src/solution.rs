use std::collections::*;
use std::io::Read;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    let mut vm = ic::Icvm::new(prog);

    let mut map = HashMap::new();
    let mut p = (0, 0);
    map.insert(p, 9);
    while vm.status() != ic::Status::Finished {
        // let input = loop {
        //     let mut ch = String::new();
        //     std::io::stdin().read_line(&mut ch).unwrap();
        //     match ch.trim() {
        //         "w" => break 1,
        //         "s" => break 2,
        //         "a" => break 3,
        //         "d" => break 4,
        //         _ => (),
        //     }
        // };
        let input = rand::random::<u8>() % 4 + 1;
        let newp = match input {
            1 => (p.0, p.1+1),
            2 => (p.0, p.1-1),
            3 => (p.0-1, p.1),
            4 => (p.0+1, p.1),
            _ => panic!("aaa"),
        };
        vm.push_input(input as i128);
        vm.run();
        match vm.drain_outputs().next().unwrap() {
            0 => { map.insert(newp, 2); },
            1 => { map.insert(newp, 1); p = newp; },
            2 => { println!("Found oxygen"); map.insert(newp, 3); p = newp; break; },
            _ => panic!("aaaaaa"),
        }
        // print_map(&map, p);
    }
    print_map(&map, p);
    0
}
fn print_map(map: &HashMap<(isize, isize), usize>, p: (isize, isize)) {
    if map.is_empty() { return }
    println!("----------------------------------");
    let minx = map.iter().map(|(a, _)| a.0).min().unwrap().min(p.0);
    let maxx = map.iter().map(|(a, _)| a.0).max().unwrap().max(p.0);
    let miny = map.iter().map(|(a, _)| a.1).min().unwrap().min(p.1);
    let maxy = map.iter().map(|(a, _)| a.1).max().unwrap().max(p.1);

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if p == (x, y) {
                print!("@");
            } else if (0, 0) == (x, y) {
                print!("X");
            } else {
                match map.get(&(x, y)).unwrap_or(&0) {
                    0 => print!("?"),
                    1 => print!(" "),
                    2 => print!("{}", match (map.get(&(x+1, y)).unwrap_or(&0),
                                map.get(&(x-1, y)).unwrap_or(&0),
                                map.get(&(x, y+1)).unwrap_or(&0),
                                map.get(&(x, y-1)).unwrap_or(&0)) {
                        (2, 2, 2, 2) => "┼",
                        (2, 2, 2, _) => "┴",
                        (2, 2, _, 2) => "┬",
                        (2, _, 2, 2) => "├",
                        (_, 2, 2, 2) => "┤",
                        (2, 2, _, _) => "─",
                        (2, _, 2, _) => "└",
                        (_, 2, 2, _) => "┘",
                        (2, _, _, 2) => "┌",
                        (_, 2, _, 2) => "┐",
                        (_, _, 2, 2) => "│",
                        (_, _, _, 2) => "│",
                        (_, _, 2, _) => "│",
                        (_, 2, _, _) => "─",
                        (2, _, _, _) => "─",
                        (_, _, _, _) => "☐",
                    }),
                    3 => print!("O"),
                    4 => print!("."),
                    _ => print!(" "),
                }
            }
        }
        println!()
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    let mut vm = ic::Icvm::new(prog);

    let mut map = HashMap::new();
    let mut p = (0, 0);
    map.insert(p, 1);
    visit(&mut vm, &mut map, p, 1);
    // print_map(&map, p);

    // flood fill

    let mut q = VecDeque::new();
    q.push_front((0, *map.iter().find(|(p, e)| e == &&3).unwrap().0));

    let mut max = 0;
    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        for (x, y) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
            if map.get(&(i.1 .0 + x, i.1 .1 + y)).unwrap_or(&2) != &2 {
                q.push_back((i.0 + 1, (i.1 .0 + x, i.1 .1 + y)));
                *map.get_mut(&(i.1 .0 + x, i.1 .1 + y)).unwrap() = 2;
            }
        }
        max = max.max(i.0);
        // dbg!(i.0);
    }

    max
}

fn visit(vm: &mut ic::Icvm, map: &mut HashMap<(isize, isize), usize>, p: (isize, isize), depth: usize) {
    let pr = map[&p];
    *map.get_mut(&p).unwrap() = 4;
    for input in 1..=4 {

        let newp = match input {
            1 => (p.0, p.1+1),
            2 => (p.0, p.1-1),
            3 => (p.0-1, p.1),
            4 => (p.0+1, p.1),
            _ => panic!("aaa"),
        };

        if map.contains_key(&newp) { continue; }

        vm.push_input(input);
        vm.run();
        match vm.drain_outputs().next().unwrap() {
            0 => { map.insert(newp, 2); continue; },
            1 => { map.insert(newp, 1); },
            2 => { map.insert(newp, 3); },
            _ => panic!("aaaaaa"),
        }
        // print_map(&map, p);
        // std::thread::sleep(std::time::Duration::from_millis(20));

        visit(vm, map, newp, depth+1);

        vm.push_input(match input {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("nooooooooooo"),
        });
        vm.run();
        vm.drain_outputs().for_each(|_| ());
        // print_map(&map, p);
        // std::thread::sleep(std::time::Duration::from_millis(20));
    }
    *map.get_mut(&p).unwrap() = pr;
}
