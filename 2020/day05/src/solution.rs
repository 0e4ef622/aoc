pub fn part1(input: &str) -> impl std::fmt::Display {
    input.as_bytes().split(|&x| x == b'\n').map(|l|
        l.iter().fold(0usize, |a, &x| a*2 + ((!x as usize & 4) >> 2))
    )
    .max()
    .unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut t = [false; 1024];
    input.as_bytes().split(|&x| x == b'\n').map(|l|
        l.iter().fold(0usize, |a, &x| a*2 + ((!x as usize & 4) >> 2))
    ).for_each(|n| t[n] = true);
    t[0] = false;
    t.iter().enumerate().skip_while(|b| !b.1).skip_while(|b| *b.1).next().unwrap().0
}
