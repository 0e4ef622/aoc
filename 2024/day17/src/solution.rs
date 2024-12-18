use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let [regs, prog] = *input.split("\n\n").cv() else { unreachable!() };
    let [mut a, mut b, mut c] = *regs.lines().map(|s| s[12..].parse::<i64>().unwrap()).cv() else { unreachable!() };

    let prog = prog[9..].trim().split(',').map(|s| s.parse::<u8>().unwrap()).cv();

    let combo = |o, a, b, c| {
        match o {
            0..=3 => o as i64,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("got combo operand {o}"),
        }
    };
    let mut pc = 0;
    let mut output = vec![];
    loop {
        let [ins, opd, ..] = prog[pc..] else { break };
        let mut jumped = false;
        match ins {
            /* adv */ 0 => a = a >> combo(opd, a, b, c),
            /* bxl */ 1 => b = b ^ opd as i64,
            /* bst */ 2 => b = combo(opd, a, b, c) % 8,
            /* jnz */ 3 => if a != 0 {
                pc = opd as usize;
                jumped = true;
            }
            /* bxc */ 4 => b = b ^ c,
            /* out */ 5 => output.push(combo(opd, a, b, c) % 8),
            /* bdv */ 6 => b = a >> combo(opd, a, b, c),
            /* cdv */ 7 => c = a >> combo(opd, a, b, c),
            _ => panic!("got instruction {ins}"),
        }
        if !jumped {
            pc += 2;
        }
    }
    output.into_iter().join(",")
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let [regs, prog] = *input.split("\n\n").cv() else { unreachable!() };
    // let [mut a, mut b, mut c] = *regs.lines().map(|s| s[12..].parse::<i64>().unwrap()).cv() else { unreachable!() };

    let prog = prog[9..].trim().split(',').map(|s| s.parse::<u8>().unwrap()).cv();

    let combo = |o| {
        match o {
            0..=3 => o.to_string(),
            4 => "a".into(),
            5 => "b".into(),
            6 => "c".into(),
            _ => panic!("got combo operand {o}"),
        }
    };
    let mut pc = 0;
    // let mut output = vec![];
    loop {
        let [ins, opd, ..] = prog[pc..] else { break };
        println!("{}", match ins {
            /* adv */ 0 => format!("a = a >> {}", combo(opd)),
            /* bxl */ 1 => format!("b = b ^ {opd}"),
            /* bst */ 2 => format!("b = {} % 8", combo(opd)),
            /* jnz */ 3 => format!("if a != 0: pc = {opd}"),
            /* bxc */ 4 => format!("b = b ^ c"),
            /* out */ 5 => format!("out {} % 8", combo(opd)),
            /* bdv */ 6 => format!("b = a >> {}", combo(opd)),
            /* cdv */ 7 => format!("c = a >> {}", combo(opd)),
            _ => panic!("got instruction {ins}"),
        });
        pc += 2;
    }

    // let mut a = 0;
    // for &n in prog.iter().rev() {
    //     let mut found = false;
    //     for bb in 0..8 {
    //         // let x = b^3;
    //         // let c = a>>x;
    //         // let y = x^5^c;
    //         let la = (a<<3) | bb;
    //         let b = bb^3;
    //         let c = la>>b;
    //         let b = b^5;
    //         let b = b^c;
    //         if (b%8) as u8 == n {
    //             a <<= 3;
    //             a |= bb;
    //             found = true;
    //             break;
    //         }
    //     }
    //     if !found {
    //         println!("{a}");
    //         panic!("D:");
    //     }
    // }

    find(0, &prog).unwrap()
    // output.into_iter().join(",")
}

fn find(aa: usize, prog: &[u8]) -> Option<usize> {
    let Some(&n) = prog.last() else {
        return Some(aa);
    };

    for bb in 0..8 {
        let b = bb;
        let a = aa;

        let a = (a<<3) | b;
        let b = b^3;
        let c = a>>b;
        let b = b^5;
        let b = b^c;
        if (b%8) as u8 == n {
            if let Some(r) = find((aa<<3)|bb, &prog[..prog.len()-1]) {
                return Some(r);
            }
        }
    }
    None
}
