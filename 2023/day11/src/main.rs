#![allow(warnings)]
use std::{io::Read, hint::black_box};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    if args.len() <= 1 {
        let p1 = day11::solution::part1(&input);
        println!("part 1: {}", p1);
        let p2 = day11::solution::part2(&input);
        println!("part 2: {}", p2);

        let p1 = day11::unsafe_solution::part1(&input);
        println!("part 1 unsafe: {}", p1);
        let p2 = day11::unsafe_solution::part2(&input);
        println!("part 2 unsafe: {}", p2);
    } else if args[1] == "2" {
        for i in 0..100000 {
            let p2 = day11::unsafe_solution::part2(&input);
            black_box(p2);
            // println!("{}", p2);
        }
    } else {
        let p1 = day11::solution::part1(&input);
        println!("{}", p1);
    }
}
