pub fn part1(_: &str) -> usize {
    // let mut low = [3, 6, 7, 4, 7, 9];
    // let mut hi = [8, 9, 3, 6, 9, 8];
    let mut low = [1, 1, 1, 1, 1, 1];
    let mut hi = [9, 9, 9, 9, 9, 9];


    // input preprocessing
    for i in 0..5 {
        if low[i] > low[i+1] {
            let n = low[i];
            low[i..].iter_mut().for_each(|x| *x = n);
            break;
        }
    }
    for i in 1..6 {
        if hi[i-1] > hi[i] {
            hi[i-1] -= 1;
            hi[i..].iter_mut().for_each(|x| *x = 9);
            break;
        }
    }

    let mut c = 0;

    for d1 in low[0]..=hi[0] {
        let a = d1 == low[0];
        let b = d1 == hi[0];
        let x = if a { low[1] } else { d1 };
        let y = if b { hi[1] } else { 9 };
        for d2 in x..=y {
            let a = a && d2 == low[1];
            let b = b && d2 == hi[1];
            let x = if a { low[2] } else { d2 };
            let y = if b { hi[2] } else { 9 };
            for d3 in x..=y {
                let a = a && d3 == low[2];
                let b = b && d3 == hi[2];
                let x = if a { low[3] } else { d3 };
                let y = if b { hi[3] } else { 9 };
                for d4 in x..=y {
                    let a = a && d4 == low[3];
                    let b = b && d4 == hi[3];
                    let x = if a { low[4] } else { d4 };
                    let y = if b { hi[4] } else { 9 };
                    for d5 in x..=y {
                        let a = a && d5 == low[4];
                        let b = b && d5 == hi[4];
                        let x = if a { low[5] } else { d5 };
                        let y = if b { hi[5] } else { 9 };
                        for d6 in x..=y {

                            c += (d1 == d2 ||
                                  d2 == d3 ||
                                  d3 == d4 ||
                                  d4 == d5 ||
                                  d5 == d6) as usize;
                        }
                    }
                }
            }
        }
    }

    c
}
pub fn part2(_: &str) -> usize {
    // let mut low = [3, 6, 7, 4, 7, 9];
    // let mut hi = [8, 9, 3, 6, 9, 8];
    let mut low = [1, 1, 1, 1, 1, 1];
    let mut hi = [9, 9, 9, 9, 9, 9];


    // input preprocessing
    for i in 0..5 {
        if low[i] > low[i+1] {
            let n = low[i];
            low[i..].iter_mut().for_each(|x| *x = n);
            break;
        }
    }
    for i in 1..6 {
        if hi[i-1] > hi[i] {
            hi[i-1] -= 1;
            hi[i..].iter_mut().for_each(|x| *x = 9);
            break;
        }
    }

    let mut c = 0;

    for d1 in low[0]..=hi[0] {
        let a = d1 == low[0];
        let b = d1 == hi[0];
        let x = if a { low[1] } else { d1 };
        let y = if b { hi[1] } else { 9 };
        for d2 in x..=y {
            let a = a && d2 == low[1];
            let b = b && d2 == hi[1];
            let x = if a { low[2] } else { d2 };
            let y = if b { hi[2] } else { 9 };
            for d3 in x..=y {
                let a = a && d3 == low[2];
                let b = b && d3 == hi[2];
                let x = if a { low[3] } else { d3 };
                let y = if b { hi[3] } else { 9 };
                for d4 in x..=y {
                    let a = a && d4 == low[3];
                    let b = b && d4 == hi[3];
                    let x = if a { low[4] } else { d4 };
                    let y = if b { hi[4] } else { 9 };
                    for d5 in x..=y {
                        let a = a && d5 == low[4];
                        let b = b && d5 == hi[4];
                        let x = if a { low[5] } else { d5 };
                        let y = if b { hi[5] } else { 9 };
                        for d6 in x..=y {

                            c += (d1 == d2 && d2 != d3 ||
                                  d2 == d3 && d3 != d4 && d1 != d2 ||
                                  d3 == d4 && d4 != d5 && d2 != d3 ||
                                  d4 == d5 && d5 != d6 && d3 != d4 ||
                                  d5 == d6 && d4 != d5) as usize;
                        }
                    }
                }
            }
        }
    }
    c
}
