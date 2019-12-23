use std::collections::*;
use rand::random;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut vms: Vec<_> = (0..50).map(|n| { let mut vm = ic::Icvm::new(prog.clone()); vm.push_input(n); vm }).collect();

    loop {
        for vm in 0..50 {
            if vms[vm].inputs().count() == 0 {
                vms[vm].push_input(-1);
            }
            vms[vm].run();
            let outputs: Vec<_> = vms[vm].drain_outputs().collect();
            for output in outputs.chunks(3) {
                let addr = output[0] as usize;
                let x = output[1];
                let y = output[2];

                if addr == 255 {
                    return y;
                }

                vms[addr].push_input(x);
                vms[addr].push_input(y);
            }
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let prog = input.trim().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let mut vms: Vec<_> = (0..50).map(|n| { let mut vm = ic::Icvm::new(prog.clone()); vm.push_input(n); vm }).collect();

    let mut lastd = None;
    let mut nat = None;

    loop {
        let mut idle = true;
        for vm in 0..50 {
            if vms[vm].inputs().count() == 0 {
                vms[vm].push_input(-1);
            } else {
                idle = false;
            }
            vms[vm].run();
            let outputs: Vec<_> = vms[vm].drain_outputs().collect();
            for output in outputs.chunks(3) {
                let addr = output[0] as usize;
                let x = output[1];
                let y = output[2];

                if addr == 255 {
                    nat = Some((x, y));
                } else {
                    vms[addr].push_input(x);
                    vms[addr].push_input(y);
                }
            }
        }
        if idle {
            match (nat, lastd) {
                (Some((nx, ny)), Some((lx, ly))) if ny == ly => {
                    return ny;
                }
                _ => (),
            }
            lastd = nat;
            vms[0].push_input(nat.unwrap().0);
            vms[0].push_input(nat.unwrap().1);
        }
    }
    0
}
