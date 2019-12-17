use std::collections::*;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut botp = (0i128, 0i128);
    let mut botd = 0i32; // 0 = up, 1 = right, 2 = down, 3 = left
    let mut vm = ic::Icvm::new(input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect());
    let mut paint = HashMap::new();

    while vm.status() != ic::Status::Finished {
        vm.run();

        let mut flip = true;
        for out in vm.drain_outputs() {
            if flip {
                paint.insert(botp, out);
                flip = false;
            } else {
                match out {
                    0 => botd = (botd - 1).rem_euclid(4),
                    1 => botd = (botd + 1).rem_euclid(4),
                    _ => panic!("aaaa"),
                }

                match botd {
                    0 => botp.1 += 1,
                    1 => botp.0 += 1,
                    2 => botp.1 -= 1,
                    3 => botp.0 -= 1,
                    _ => panic!("aaa"),
                }

                flip = true;
            }
        }
        vm.push_input(*paint.get(&botp).unwrap_or(&0));
    }

    paint.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut botp = (0i128, 0i128);
    let mut botd = 0i32; // 0 = up, 1 = right, 2 = down, 3 = left
    let mut vm = ic::Icvm::new(input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect());
    let mut paint = HashMap::new();
    paint.insert((0, 0), 0);

    while vm.status() != ic::Status::Finished {
        vm.run();

        let mut flip = true;
        for out in vm.drain_outputs() {
            if flip {
                paint.insert(botp, out);
                flip = false;
            } else {
                match out {
                    0 => botd = (botd - 1).rem_euclid(4),
                    1 => botd = (botd + 1).rem_euclid(4),
                    _ => panic!("aaaa"),
                }

                match botd {
                    0 => botp.1 += 1,
                    1 => botp.0 += 1,
                    2 => botp.1 -= 1,
                    3 => botp.0 -= 1,
                    _ => panic!("aaa"),
                }

                flip = true;
            }
        }
        vm.push_input(*paint.get(&botp).unwrap_or(&0));
    }

    let minx = paint.iter().map(|(a, _)| a.0).min().unwrap();
    let maxx = paint.iter().map(|(a, _)| a.0).max().unwrap();
    let miny = paint.iter().map(|(a, _)| a.1).min().unwrap();
    let maxy = paint.iter().map(|(a, _)| a.1).max().unwrap();

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            match paint.get(&(x, y)).unwrap_or(&0) {
                0 => print!(" "),
                1 => print!("█"),
                _ => panic!("aa"),
            }
        }
        println!()
    }

    0
}
