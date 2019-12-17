use ic::{Icvm, Status};
pub fn part1(input: &str) -> i128 {
    // part2(input)
    0
}

pub fn part2(input: &str) -> i128 {
    let program = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    let vm = Icvm::new(program);

    let mut max = 0i128;
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

                        let mut output = 0;

                        let mut vm1 = vm.clone();
                        let mut vm2 = vm.clone();
                        let mut vm3 = vm.clone();
                        let mut vm4 = vm.clone();
                        let mut vm5 = vm.clone();

                        vm1.push_input(a);
                        vm1.push_input(0);
                        vm2.push_input(b);
                        vm3.push_input(c);
                        vm4.push_input(d);
                        vm5.push_input(e);

                        loop {

                            let mut alldone = true;
                            for vm in &mut [&mut vm1, &mut vm2, &mut vm3, &mut vm4, &mut vm5] {
                                unsafe { vm.run_fast_unchecked(); }
                                alldone = alldone && vm.status() == Status::Finished;
                            }

                            vm1.push_inputs(vm5.drain_outputs());
                            vm2.push_inputs(vm1.drain_outputs());
                            vm3.push_inputs(vm2.drain_outputs());
                            vm4.push_inputs(vm3.drain_outputs());
                            vm5.push_inputs(vm4.drain_outputs());

                            if alldone { break; }
                        }

                        max = max.max(vm1.inputs().next().unwrap());
                    }
                }
            }
        }
    }
    max
}
