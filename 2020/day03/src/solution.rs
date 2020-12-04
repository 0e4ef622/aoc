pub fn part1(i: &[u8]) -> usize {
    (0..i.len())
        .filter(|&n| (n + n / 32 * 3 / 31 * 31) % 35 == 0 && i[n] == 35)
        .count()
}
pub fn oldpart1(input: &str) -> impl std::fmt::Display {
    input.lines().enumerate().fold(0, |a, x| {
        a + (x.1.as_bytes()[x.0 * 3 % x.1.len()] == b'#') as u32
    })
}

pub fn part2(i: &[u8]) -> impl std::fmt::Display {
    (|f: &dyn Fn(_, _) -> _| f(1, 1) * f(3, 1) * f(5, 1) * f(7, 1) * f(1, 2))(&|x, y| {
        (0..i.len())
            .filter(|&n| (n + n / 32 / y * x / 31 * 31) % (32 * y + x) == 0 && i[n] == 35)
            .count()
    })
}

pub fn oldpart2(input: &str) -> impl std::fmt::Display {
    (|f: &dyn Fn(_, _) -> _| f(1, 1) * f(3, 1) * f(5, 1) * f(7, 1) * f(1, 2))(&|x, y| {
        input.lines().step_by(y).enumerate().fold(0, |a, (i, l)| {
            a + (l.as_bytes()[i * x % l.len()] == b'#') as usize
        })
    })
}
