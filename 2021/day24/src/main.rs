#![allow(warnings)]
use std::io::Read;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    if args.len() <= 1 {
        let p1 = day24::solution::part1(&input);
        println!("part 1: {}", p1);
        let p2 = day24::solution::part2(&input);
        println!("part 2: {}", p2);
    } else if args[1] == "2" {
        let p2 = day24::solution::part2(&input);
        println!("{}", p2);
    } else {
        let p1 = day24::solution::part1(&input);
        println!("{}", p1);
    }
}
