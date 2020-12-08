pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().map(|x| (false, x)).collect::<Vec<_>>();
    let mut acc = 0;
    let mut i: isize = 0;
    loop {
        let (v, line) = &mut lines[i as usize];
        if *v { break acc; }
        *v = true;
        let n = line[4..].parse::<isize>().unwrap();
        match &line[..3] {
            "jmp" => i += n,
            "acc" => { acc += n; i += 1; },
            _ => i += 1,
        }
    }
}

fn parse_num(s: &[u8]) -> isize {
    let mut r = 0;
    for b in &s[1..] {
        r = 10*r + (b - b'0') as isize;
    }
    if s[0] == b'-' {
        -r
    } else {
        r
    }
}

// graph based approach
pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let lines = input.split(|&x| x == b'\n').filter(|&x| !x.is_empty());
    let len = lines.clone().count();
    let instructions = lines.map(|x| (&x[..3], parse_num(&x[4..])));

    // accumulator, offset1, offset2, visited
    let mut graph = vec![(0, 1, None, false); len*2];
    // left half represents the program pre-modified, right half post-modified
    let (lh, rh) = graph.split_at_mut(len);
    for (i, (op, val)) in instructions.enumerate() {
        match op[0] {
            b'a' => {
                lh[i].0 = val;
                rh[i].0 = val;
            }
            b'j' => {
                lh[i].1 = val;
                rh[i].1 = val;
                lh[i].2 = Some(len as isize + 1);
            }
            b'n' => lh[i].2 = Some(len as isize + val),
            _ => unreachable!(),
        }
    }

    // depth first search
    let mut stack = vec![(0, 0isize)];
    while let Some((mut acc, pos)) = stack.pop() {
        if pos as usize == 2*len { return acc; }
        let (accc, offset1, offset2, visited) = &mut graph[pos as usize];
        if *visited { continue; }
        *visited = true;
        acc += *accc;
        stack.push((acc, pos + *offset1));
        if let Some(offset2) = *offset2 {
            stack.push((acc, pos + offset2));
        }
    }
    panic!("answer not found")
}

pub fn oldpart2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines().map(|x| (false, x)).collect::<Vec<_>>();
    for k in 0..lines.len() {
        let mut acc = 0;
        let mut i: isize = 0;
        lines.iter_mut().for_each(|x| x.0=false);
        let r = loop {
            let (v, line) = &mut lines[i as usize];
            if *v { break false; }
            *v = true;
            let n = line[4..].parse::<isize>().unwrap();
            match (&line[..3], i == k as isize) {
                ("jmp", false) | ("nop", true) => i += n,
                ("acc", _) => { acc += n; i += 1; },
                _ => i += 1,
            }
            if i as usize == lines.len() { break true; }
        };
        if r { return acc; }
    }
    0
}
