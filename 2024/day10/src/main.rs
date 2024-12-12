#![allow(warnings)]
use std::io::Read;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    if args.len() <= 1 {
        let p1 = day10::solution::part1(&input);
        println!("part 1: {}", p1);
        let p2 = day10::solution::part2(&input);
        println!("part 2: {}", p2);
        let elf2 = day10::elf2::run(&input);
        println!("part 2 elf: {}", p2);
    } else if args[1] == "2" {
        let p2 = day10::solution::part2(&input);
        println!("{}", p2);
    } else if args[1] == "1" {
        let p1 = day10::solution::part1(&input);
        println!("{}", p1);
    } else if args[1] == "elfloop" {
        for i in 0..50000 {
            let elf2 = day10::elf2::run(&input);
            std::hint::black_box(elf2);
        }
    }
}
