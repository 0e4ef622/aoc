pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut c: usize = 0;
    for group in input.trim().split("\n\n") {
        let mut a = [false; 26];
        for line in group.lines() {
            line.bytes().for_each(|c| a[(c-b'a') as usize] = true);
        }
        c += a.iter().copied().map(|x| x as usize).sum::<usize>();
    }
    c
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut c: usize = 0;
    for group in input.trim().split("\n\n") {
        let mut b = [true; 26];
        for line in group.lines() {
            let mut a = [false; 26];
            line.bytes().for_each(|c| a[(c-b'a') as usize] = true);
            a.iter().zip(&mut b).for_each(|(a, b)| *b &= *a);
        }
        c += b.iter().copied().map(|x| x as usize).sum::<usize>();
    }
    c
}
