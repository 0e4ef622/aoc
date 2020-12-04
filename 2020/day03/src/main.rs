#![feature(iter_map_while)]
#![allow(warnings)]
use std::io::Read;
mod solution;
// const INPUT: &'static str = include_str!("../in");
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);
    let p1 = solution::part1(input.as_bytes());
    println!("part 1: {}", p1);
    let p2 = solution::part2(input.as_bytes());
    println!("part 2: {}", p2);
}
