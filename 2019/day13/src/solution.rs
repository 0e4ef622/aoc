use std::collections::*;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    let mut vm = ic::Icvm::new(prog);
    vm.run();
    let mut map = HashMap::new();
    let a = vm.drain_outputs().collect::<Vec<_>>();
    for ch in a.chunks(3) {
        map.insert((ch[0], ch[1]), ch[2]);
    }
    // let minx = map.iter().map(|(a, _)| a.0).min().unwrap();
    // let maxx = map.iter().map(|(a, _)| a.0).max().unwrap();
    // let miny = map.iter().map(|(a, _)| a.1).min().unwrap();
    // let maxy = map.iter().map(|(a, _)| a.1).max().unwrap();

    // for y in (miny..=maxy).rev() {
    //     for x in minx..=maxx {
    //         match map.get(&(x, y)).unwrap_or(&0) {
    //             0 => print!(" "),
    //             1 => print!("█"),
    //             2 => print!("#"),
    //             3 => print!("-"),
    //             4 => print!("O"),
    //             _ => print!(" "),
    //         }
    //     }
    //     println!()
    // }
    map.into_iter().filter(|(a, b)| b == &2).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut prog = input.trim().split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    prog[0] = 2;
    let mut vm = ic::Icvm::new(prog);
    let mut map = HashMap::new();
    let mut score = 0;
    while vm.status() != ic::Status::Finished {
        vm.run();
        let a = vm.drain_outputs().collect::<Vec<_>>();
        for ch in a.chunks(3) {
            map.insert((ch[0], ch[1]), ch[2]);

            if ch[0] == -1 && ch[1] == 0 {
                score = ch[2];
            }
        }
        // println!("score = {}", score);

        // let minx = map.iter().map(|(a, _)| a.0).min().unwrap();
        // let maxx = map.iter().map(|(a, _)| a.0).max().unwrap();
        // let miny = map.iter().map(|(a, _)| a.1).min().unwrap();
        // let maxy = map.iter().map(|(a, _)| a.1).max().unwrap();

        // for y in (miny..=maxy).rev() {
        //     for x in minx..=maxx {
        //         match map.get(&(x, y)).unwrap_or(&0) {
        //             0 => print!(" "),
        //             1 => print!("█"),
        //             2 => print!("#"),
        //             3 => print!("-"),
        //             4 => print!("O"),
        //             _ => print!(" "),
        //         }
        //     }
        //     println!()
        // }
        // std::thread::sleep(std::time::Duration::from_millis(50));
        let ballp = map.iter().find(|(a, c)| c == &&4).unwrap().0;
        let barp = map.iter().find(|(a, c)| c == &&3).unwrap().0;
        if ballp.0 < barp.0 {
            vm.push_input(-1);
        } else if ballp.0 > barp.0 {
            vm.push_input(1);
        } else {
            vm.push_input(0);
        }
        // loop {
        //     let mut s = String::new();
        //     std::io::stdin().read_line(&mut s);
        //     match s.trim() {
        //         "a" => { vm.push_input(-1); break; }
        //         "" => { vm.push_input(0); break; }
        //         "d" => { vm.push_input(1); break; }
        //         _ => println!("bad input"),
        //     }
        // }
    }
    score
}
