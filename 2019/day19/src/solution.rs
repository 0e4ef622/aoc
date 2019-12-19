use std::collections::*;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut vm = ic::Icvm::new(prog.clone());
            vm.push_input(x);
            vm.push_input(y);
            vm.run();
            count += vm.pop_output().unwrap();
        }
    }
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut y = 100;
    let mut px = 0;
    let mut ex = 0;
    loop {
        let mut x = px;
        loop {
            let mut vm = ic::Icvm::new(prog.clone());
            vm.push_input(x);
            vm.push_input(y);
            vm.run();
            if vm.pop_output().unwrap() == 1 { break; }
            x += 1;
        }
        px = x;
        x = x.max(ex);
        loop {
            let mut vm = ic::Icvm::new(prog.clone());
            vm.push_input(x);
            vm.push_input(y);
            vm.run();
            if vm.pop_output().unwrap() == 0 { break; }
            x += 1;
        }
        ex = x;
        x -= 1;

        let mut vm = ic::Icvm::new(prog.clone());
        vm.push_input(x-99);
        vm.push_input(y+99);
        vm.run();

        if vm.pop_output().unwrap() == 1 { break (x-99)*10000+y; }
        y += 1;
    }
}
