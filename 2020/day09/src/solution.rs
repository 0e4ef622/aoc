fn parse_num(s: &[u8]) -> isize {
    let mut r = 0;
    for b in s {
        r *= 10;
        r += (b - b'0') as isize;
    }
    r
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let n = input.lines().map(str::as_bytes).map(parse_num).collect::<Vec<_>>();

    (25..n.len()).find(|&i| {
        !(i-25..i).any(|a| {
            (a..i).any(|b| n[a] + n[b] == n[i])
        })
    })
    .map(|i| n[i]).unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let n = input.lines().map(str::as_bytes).map(parse_num).collect::<Vec<_>>();

    let f = (25..n.len()).find(|&i| {
        !(i-25..i).any(|a| {
            (a..i).any(|b| n[a] + n[b] == n[i])
        })
    }).unwrap();

    let mut p = n.clone();
    for i in 1..p.len() {
        p[i] += p[i-1];
    }

    let (a, b) = (0..f)
        .flat_map(|a| (a+1..f).map(move |b| (a, b)))
        .find(|&(a, b)| p[b] - p[a] == n[f])
        .unwrap();

    let max = n[a..=b].iter().max().unwrap();
    let min = n[a..=b].iter().min().unwrap();
    max + min
}
