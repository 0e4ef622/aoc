pub fn part1(input: &str) -> usize {
    let mut nums = input.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    nums[1] = 12;
    nums[2] = 2;
    let mut ci = 0;
    while nums[ci] != 99 {
        match nums[ci] {
            1 => {
                let i = nums[ci+3];
                nums[i] = nums[nums[ci+1]]+nums[nums[ci+2]];
                ci+=4;
            }
            2 => {
                let i = nums[nums[ci+3]];
                nums[i] = nums[nums[ci+1]]*nums[nums[ci+2]];
                ci+=4;
            }
            _ => (),
        }
    }

    nums[0]

}

pub fn part2(input: &str) -> usize {
    let mut nums = input.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    for i in 0..=99 {
        for j in 0..=99 {
            let mut nums = nums.clone();
            println!("trying {} {}", i, j);
            nums[1] = i;
            nums[2] = j;
            let mut ci = 0;
            while nums[ci] != 99 {
                match nums[ci] {
                    1 => {
                        let i = nums[ci+3];
                        nums[i] = nums[nums[ci+1]]+nums[nums[ci+2]];
                        ci+=4;
                    }
                    2 => {
                        let i = nums[nums[ci+3]];
                        nums[i] = nums[nums[ci+1]]*nums[nums[ci+2]];
                        ci+=4;
                    }
                    _ => (),
                }
            }

            if nums[0] == 19690720 {
                println!("{} {}", i, j);
                return 0;
            }
        }
    }
    0
}
