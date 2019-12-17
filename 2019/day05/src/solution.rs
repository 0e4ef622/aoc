pub fn part1(input: &str) -> isize {
    0
    // let mut nums = input.trim().split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    // // nums[1] = 12;
    // // nums[2] = 2;
    // let mut ci = 0;
    // while nums[ci] != 99 {
    //     match nums[ci] % 10 {
    //         1 => {
    //             let mut a = nums[ci+1];
    //             let mut b = nums[ci+2];
    //             let mut c = nums[ci+3];

    //             let m = nums[ci] / 100 % 10;
    //             let n = nums[ci] / 1000 % 10;

    //             // println!("{:?}", &nums[ci..ci+4]);
    //             if m == 0 { a = nums[a as usize]; }
    //             if n == 0 { b = nums[b as usize]; }
    //             // println!("{} {} {}", a, b, c);

    //             nums[c as usize] = a + b;
    //             // println!("= {}", nums[c as usize]);
    //             ci+=4;
    //         }
    //         2 => {
    //             let mut a = nums[ci+1];
    //             let mut b = nums[ci+2];
    //             let mut c = nums[ci+3];

    //             let m = nums[ci] / 100 % 10;
    //             let n = nums[ci] / 1000 % 10;

    //             // println!("{:?}", &nums[ci..ci+4]);
    //             if m == 0 { a = nums[a as usize]; }
    //             if n == 0 { b = nums[b as usize]; }

    //             nums[c as usize] = a * b;
    //             // println!("= {}", nums[c as usize]);
    //             ci+=4;
    //         }
    //         3 => {
    //             let a = nums[ci+1];
    //             nums[a as usize] = 1; //input
    //             ci += 2;
    //         }
    //         4 => {
    //             let mut a = nums[ci+1];
    //             let m = nums[ci] / 100 % 10;
    //             if m == 0 { a = nums[a as usize]; }
    //             println!("{}", a);
    //             ci += 2;
    //         }
    //         _ => (),
    //     }
    // }

    // nums[0]

}

pub fn part2(input: &str) -> isize {
    let mut nums = input.trim().split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let mut ci = 0;
    while nums[ci] != 99 {
        println!("ci {}", ci);
        println!("{:?}", &nums[ci..ci+4]);
        match nums[ci] % 10 {
            1 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let mut c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                // println!("{:?}", &nums[ci..ci+4]);
                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }
                // println!("{} {} {}", a, b, c);

                nums[c as usize] = a + b;
                // println!("= {}", nums[c as usize]);
                ci+=4;
            }
            2 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let mut c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                // println!("{:?}", &nums[ci..ci+4]);
                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }

                nums[c as usize] = a * b;
                // println!("= {}", nums[c as usize]);
                ci+=4;
            }
            3 => {
                let a = nums[ci+1];
                nums[a as usize] = 5; //input
                ci += 2;
            }
            4 => {
                let mut a = nums[ci+1];
                let m = nums[ci] / 100 % 10;
                if m == 0 { a = nums[a as usize]; }
                println!("{}", a);
                ci += 2;
            }

            5 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }
                if a != 0 {
                    ci = b as usize;
                } else {
                    ci += 3;
                }
            }
            6 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }
                if a == 0 {
                    ci = b as usize;
                } else {
                    ci += 3;
                }
            }
            7 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }
                if a < b {
                    nums[c as usize] = 1;
                } else {
                    nums[c as usize] = 0;
                }

                ci += 4;
            }
            8 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }
                if a == b {
                    nums[c as usize] = 1;
                } else {
                    nums[c as usize] = 0;
                }

                ci += 4;
            }
            _ => (),
        }
    }

    nums[0]

}
