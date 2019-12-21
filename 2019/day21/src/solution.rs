use std::collections::*;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let mut vm = ic::Icvm::new(prog);

    let input = "NOT C J
NOT A T
OR T J
AND D J
WALK
";
    vm.push_inputs(input.bytes());
    vm.run();

    // for ch in vm.outputs() {
    //     print!("{}", ch as u8 as char);
    // }
    vm.outputs().last().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let mut vm = ic::Icvm::new(prog);

    let input = "OR E J
OR H J
AND D J
OR A T
AND B T
AND C T
NOT T T
AND T J
RUN
";
    vm.push_inputs(input.bytes());
    vm.run();

    // for ch in vm.outputs() {
    //     print!("{}", ch as u8 as char);
    // }
    vm.outputs().last().unwrap()
}
