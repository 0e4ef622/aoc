pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut output = 0;
    let mut prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let mut vm = ic::Icvm::new(prog);
    vm.push_input(1);
    vm.run();
    let o = vm.outputs().next().unwrap();
    o
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut output = 0;
    let mut prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();
    let mut vm = ic::Icvm::new(prog);
    vm.push_input(2);
    vm.run();
    let o = vm.outputs().next().unwrap();
    o
}
