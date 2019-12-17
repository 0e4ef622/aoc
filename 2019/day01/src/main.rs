mod solution;
const INPUT: &'static str = include_str!("../in");
fn main() {
    let p1 = solution::part1(INPUT);
    println!("part 1: {}", p1);
    let p2 = solution::part2(INPUT);
    println!("part 2: {}", p2);
}
