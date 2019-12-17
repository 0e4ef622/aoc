pub fn part1(input: &str) -> isize {
    let mut max = 0isize;
    let mut phases = [0,0,0,0,0];
    for a in 0..5 {
        for b in 0..5 {
            if a==b { continue;}
            for c in 0..5 {
                if c==a { continue;}
                if c==b { continue;}
                for d in 0..5 {
                    if d==a { continue;}
                    if d==b { continue;}
                    if d==c { continue;}
                    for e in 0..5 {
                        if e==a { continue;}
                        if e==b { continue;}
                        if e==c { continue;}
                        if e==d { continue;}

                        let mut out = 0isize;
                        run(input.trim(), &[a, 0], &mut out);
                        run(input.trim(), &[b, out], &mut out);
                        run(input.trim(), &[c, out], &mut out);
                        run(input.trim(), &[d, out], &mut out);
                        run(input.trim(), &[e, out], &mut out);
                        if out > max {
                            max = out;
                            phases = [a,b,c,d,e];
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", phases);
    max
}

pub fn part2(input: &str) -> isize {
    let mut max = 0isize;
    let mut phases = [0,0,0,0,0];
    for a in 5..10 {
        for b in 5..10 {
            if a==b { continue;}
            for c in 5..10 {
                if c==a { continue;}
                if c==b { continue;}
                for d in 5..10 {
                    if d==a { continue;}
                    if d==b { continue;}
                    if d==c { continue;}
                    for e in 5..10 {
                        if e==a { continue;}
                        if e==b { continue;}
                        if e==c { continue;}
                        if e==d { continue;}

                        let mut inputs = [
                            vec![a, 0],
                            vec![b],
                            vec![c],
                            vec![d],
                            vec![e],
                        ];

                        let mem = input.trim().split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
                        let mut mem = [
                            mem.clone(),
                            mem.clone(),
                            mem.clone(),
                            mem.clone(),
                            mem,
                        ];

                        let mut pc = [0; 5];
                        let mut cp = 0;

                        while !pc.iter().enumerate().all(|(i,&x)| mem[i][x as usize] == 99) {

                            // dbg!(cp);
                            // dbg!(pc);
                            // dbg!(&mem);
                            loop {
                                match mem[cp][pc[cp]] % 100 {
                                    1 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];
                                        let c = mem[cp][pc[cp]+3];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }

                                        mem[cp][c as usize] = a + b;
                                        pc[cp]+=4;
                                    }
                                    2 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];
                                        let c = mem[cp][pc[cp]+3];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }

                                        mem[cp][c as usize] = a * b;
                                        pc[cp]+=4;
                                    }
                                    3 => {
                                        let a = mem[cp][pc[cp]+1];
                                        if inputs[cp].is_empty() { break; }
                                        mem[cp][a as usize] = inputs[cp][0];
                                        // inputs[cp] = &inputs[cp][1..];
                                        inputs[cp].remove(0);
                                        pc[cp] += 2;
                                    }
                                    4 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        if m == 0 { a = mem[cp][a as usize]; }
                                        inputs[(cp+1)%5].push(a);
                                        pc[cp] += 2;
                                    }

                                    5 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }
                                        if a != 0 {
                                            pc[cp] = b as usize;
                                        } else {
                                            pc[cp] += 3;
                                        }
                                    }
                                    6 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }
                                        if a == 0 {
                                            pc[cp] = b as usize;
                                        } else {
                                            pc[cp] += 3;
                                        }
                                    }
                                    7 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];
                                        let c = mem[cp][pc[cp]+3];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }
                                        if a < b {
                                            mem[cp][c as usize] = 1;
                                        } else {
                                            mem[cp][c as usize] = 0;
                                        }

                                        pc[cp] += 4;
                                    }
                                    8 => {
                                        let mut a = mem[cp][pc[cp]+1];
                                        let mut b = mem[cp][pc[cp]+2];
                                        let c = mem[cp][pc[cp]+3];

                                        let m = mem[cp][pc[cp]] / 100 % 10;
                                        let n = mem[cp][pc[cp]] / 1000 % 10;

                                        if m == 0 { a = mem[cp][a as usize]; }
                                        if n == 0 { b = mem[cp][b as usize]; }
                                        if a == b {
                                            mem[cp][c as usize] = 1;
                                        } else {
                                            mem[cp][c as usize] = 0;
                                        }

                                        pc[cp] += 4;
                                    }
                                    99 => break,
                                    _ => panic!("Bad opcode"),
                                }
                            }
                            cp = (cp+1)%5;
                        }

                        if inputs[0][0] > max {
                            max = inputs[0][0];
                            phases = [a,b,c,d,e];
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", phases);
    max
}

pub fn run(input: &str, mut inputs: &[isize], output: &mut isize) {
    let mut nums = input.trim().split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let mut ci = 0;
    loop {
        match nums[ci] % 100 {
            1 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }

                nums[c as usize] = a + b;
                ci+=4;
            }
            2 => {
                let mut a = nums[ci+1];
                let mut b = nums[ci+2];
                let c = nums[ci+3];

                let m = nums[ci] / 100 % 10;
                let n = nums[ci] / 1000 % 10;

                if m == 0 { a = nums[a as usize]; }
                if n == 0 { b = nums[b as usize]; }

                nums[c as usize] = a * b;
                ci+=4;
            }
            3 => {
                let a = nums[ci+1];
                nums[a as usize] = inputs[0];
                inputs = &inputs[1..];
                ci += 2;
            }
            4 => {
                let mut a = nums[ci+1];
                let m = nums[ci] / 100 % 10;
                if m == 0 { a = nums[a as usize]; }
                *output = a;
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
            99 => break,
            _ => panic!("Bad opcode"),
        }
    }
}
