use std::collections::*;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut nums = input.trim().bytes().map(|x| (x - b'0') as i64).skip(0).collect::<Vec<_>>();

    (0..100).for_each(|_| next_phase(&mut nums, 0));
    nums[..8].iter().fold(0i64, |a, n| a*10+n)
}

fn ix(n: usize, p: usize) -> usize {
    ((n+1) / p) % 4
}

fn next_phase(nums: &mut [i64], strt: usize) {
    let mut pfs = vec![0];
    pfs.extend(&*nums);
    for i in 1..pfs.len() {
        pfs[i] += pfs[i-1];
    }

    for n in strt..nums.len()+strt {

        let mut newn = 0;

        let mut j = n;
        while j < nums.len()+strt {
            newn += pfs[(j+n-strt+1).min(pfs.len()-1)] - pfs[j-strt];
            j += 4*(n+1);
        }

        let mut j = 2*(n+1)+n;
        while j < nums.len()+strt {
            newn -= pfs[(j+n-strt+1).min(pfs.len()-1)] - pfs[j-strt];
            j += 4*(n+1);
        }
        nums[n-strt] = newn.abs() % 10;
    }
}

// pub fn part2(input: &str) -> impl std::fmt::Display {

//     // parse input
//     let onums = input.trim().bytes().map(|x| (x - b'0') as i64).collect::<Vec<_>>();

//     // get offset
//     let offset = input[..7].parse::<usize>().unwrap();
//     // let offset = 0;

//     // calculate new input
//     let mut nums = std::iter::repeat(&onums).take(10000).flatten().skip(offset).copied().collect::<Vec<_>>();

//     // 100 phases
//     for n in 0..100 {

//         // next_phase(&mut nums, offset);

//         // reverse prefix sum
//         for i in (0..nums.len()-1).rev() {
//             nums[i] += nums[i+1];
//         }

//         nums.iter_mut().for_each(|x| *x %= 10);
//     }
//     // for n in &nums {
//     //     print!("{}", n);
//     // }
//     // println!();

//     // final answer
//     // for i in 0..8 {
//     //     print!("{}", nums[i as usize]);
//     // }
//     // println!();
//     // 0
//     nums[..8].iter().fold(0i64, |a, &n| a*10+n)
// }
pub fn part2(input: &str) -> usize {

    // parse input
    let onums = input
        .trim()
        .bytes()
        .map(|x| (x - b'0') as usize)
        .collect::<Vec<_>>();

    let offset = input[..7].parse::<usize>().unwrap();

    // calculate new input
    let mut nums = std::iter::repeat(&onums)
        .take(10000)
        .flatten()
        .copied()
        .skip(offset as usize)
        .collect::<Vec<_>>();

    // 100 phases
    for n in 0..100 {

        // reverse prefix sum
        let mut sum = 0;
        for n in nums.iter_mut().rev() {
            sum += *n;
            *n = sum % 10;
        }
    }

    // final answer
    nums[..8].iter().fold(0, |a, &n| a*10+n)
}
