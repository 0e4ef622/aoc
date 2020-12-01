use std::collections::*;
use std::io::Read;
use rand::random;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let mut vm = ic::Icvm::new(prog);
    let prein = "north\neast\ntake coin\nwest\nsouth\neast\ntake cake\n";
    vm.push_inputs(prein.bytes());

    vm.run();
    while vm.status() != ic::Status::Finished {

        for output in vm.drain_outputs() {
            print!("{}", output as u8 as char);
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        vm.push_inputs(input.bytes());
        vm.run();
    }
    for output in vm.drain_outputs() {
        print!("{}", output as u8 as char);
    }
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    0
}
