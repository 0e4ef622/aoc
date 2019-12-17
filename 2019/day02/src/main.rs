use std::io::Read;
mod solution;
// const INPUT: &'static str = include_str!("../in");
fn main() {
    let mut INPUT = String::new();
    std::io::stdin().read_to_string(&mut INPUT);
    let p1 = solution::part1(&INPUT);
    println!("part 1: {}", p1);
    let p2 = solution::part2(&INPUT);
    println!("part 2: {}", p2);
}
