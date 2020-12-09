fn parse(mut input: &[u8]) -> Vec<isize> {
    let f = || {
        if input.len() == 0 { return None; }
        let mut n = 0;
        while input[0] != b'\n' {
            n *= 10;
            n += (input[0] - b'0') as isize;
            input = &input[1..];
        }
        input = &input[1..];
        Some(n)
    };
    std::iter::from_fn(f).collect::<Vec<_>>()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let n = parse(input.as_bytes());

    (25..n.len()).find(|&i| {
        !(i-25..i).any(|a| {
            n[a+1..i].iter().any(|b| n[a] + b == n[i])
        })
    })
    .map(|i| n[i]).unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut n = parse(input.as_bytes());

    let f = (25..n.len()).find(|&i| {
        !(i-25..i).any(|a| {
            n[a+1..i].iter().any(|b| n[a] + b == n[i])
        })
    }).unwrap();

    for i in 1..n.len() {
        n[i] += n[i-1];
    }

    let mut a = 0;
    let mut b = 1;
    loop {
        let n = n[b] - n[a] - n[f] + n[f-1];
        if n == 0 {
            break;
        } else if n < 0 {
            b += 1;
        } else {
            a += 1;
        }
    }

    let max = (a..=b).map(|x| n[x] - n[x-1]).max().unwrap();
    let min = (a..=b).map(|x| n[x] - n[x-1]).min().unwrap();
    max + min
}
