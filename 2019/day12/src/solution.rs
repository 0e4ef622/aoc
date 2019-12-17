use std::collections::*;
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut moons = input.trim().lines().map(|l| {
        let mut s = l.split(",");
        let x = s.next().unwrap()[3..].parse::<isize>().unwrap();
        let y = s.next().unwrap()[3..].parse::<isize>().unwrap();
        let z = &s.next().unwrap()[3..];
        let z = z[0..z.len()-1].parse::<isize>().unwrap();
        ((x,y,z), (0isize, 0isize, 0isize))
    })
    .collect::<Vec<_>>();

    for step in 0..1000 {
        for m in 0..moons.len() {
            for n in m+1..moons.len() {
                // if m == n { continue; }

                if moons[m].0 .0 < moons[n].0 .0 {
                    moons[m].1 .0 += 1;
                    moons[n].1 .0 -= 1;
                } else if moons[m].0 .0 > moons[n].0 .0 {
                    moons[m].1 .0 -= 1;
                    moons[n].1 .0 += 1;
                }

                if moons[m].0 .1 < moons[n].0 .1 {
                    moons[m].1 .1 += 1;
                    moons[n].1 .1 -= 1;
                } else if moons[m].0 .1 > moons[n].0 .1 {
                    moons[m].1 .1 -= 1;
                    moons[n].1 .1 += 1;
                }

                if moons[m].0 .2 < moons[n].0 .2 {
                    moons[m].1 .2 += 1;
                    moons[n].1 .2 -= 1;
                } else if moons[m].0 .2 > moons[n].0 .2 {
                    moons[m].1 .2 -= 1;
                    moons[n].1 .2 += 1;
                }
            }
        }

        for m in &mut moons {
            m.0 .0 += m.1 .0;
            m.0 .1 += m.1 .1;
            m.0 .2 += m.1 .2;
        }
    }

    moons.iter().map(|(p, v)| (p.0.abs() + p.1.abs() + p.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs())).sum::<isize>()
}

fn step(moons: &mut Vec<((isize,isize,isize),(isize,isize,isize))>) {
    for m in 0..moons.len() {
        for n in m+1..moons.len() {
            if m == n { continue; }

            if moons[m].0 .0 < moons[n].0 .0 {
                moons[m].1 .0 += 1;
                moons[n].1 .0 -= 1;
            } else if moons[m].0 .0 > moons[n].0 .0 {
                moons[m].1 .0 -= 1;
                moons[n].1 .0 += 1;
            }

            if moons[m].0 .1 < moons[n].0 .1 {
                moons[m].1 .1 += 1;
                moons[n].1 .1 -= 1;
            } else if moons[m].0 .1 > moons[n].0 .1 {
                moons[m].1 .1 -= 1;
                moons[n].1 .1 += 1;
            }

            if moons[m].0 .2 < moons[n].0 .2 {
                moons[m].1 .2 += 1;
                moons[n].1 .2 -= 1;
            } else if moons[m].0 .2 > moons[n].0 .2 {
                moons[m].1 .2 -= 1;
                moons[n].1 .2 += 1;
            }
        }
    }

    for m in moons {
        m.0 .0 += m.1 .0;
        m.0 .1 += m.1 .1;
        m.0 .2 += m.1 .2;
    }
}

fn stepx(moons: &mut Vec<((isize,isize,isize),(isize,isize,isize))>) {
    for m in 0..moons.len() {
        for n in m+1..moons.len() {
            if m == n { continue; }

            if moons[m].0 .0 < moons[n].0 .0 {
                moons[m].1 .0 += 1;
                moons[n].1 .0 -= 1;
            } else if moons[m].0 .0 > moons[n].0 .0 {
                moons[m].1 .0 -= 1;
                moons[n].1 .0 += 1;
            }
        }
    }

    for m in moons {
        m.0 .0 += m.1 .0;
        m.0 .1 += m.1 .1;
        m.0 .2 += m.1 .2;
    }
}
fn stepy(moons: &mut Vec<((isize,isize,isize),(isize,isize,isize))>) {
    for m in 0..moons.len() {
        for n in m+1..moons.len() {
            if m == n { continue; }

            if moons[m].0 .1 < moons[n].0 .1 {
                moons[m].1 .1 += 1;
                moons[n].1 .1 -= 1;
            } else if moons[m].0 .1 > moons[n].0 .1 {
                moons[m].1 .1 -= 1;
                moons[n].1 .1 += 1;
            }
        }
    }

    for m in moons {
        m.0 .0 += m.1 .0;
        m.0 .1 += m.1 .1;
        m.0 .2 += m.1 .2;
    }
}
fn stepz(moons: &mut Vec<((isize,isize,isize),(isize,isize,isize))>) {
    for m in 0..moons.len() {
        for n in m+1..moons.len() {
            if m == n { continue; }

            if moons[m].0 .2 < moons[n].0 .2 {
                moons[m].1 .2 += 1;
                moons[n].1 .2 -= 1;
            } else if moons[m].0 .2 > moons[n].0 .2 {
                moons[m].1 .2 -= 1;
                moons[n].1 .2 += 1;
            }
        }
    }

    for m in moons {
        m.0 .0 += m.1 .0;
        m.0 .1 += m.1 .1;
        m.0 .2 += m.1 .2;
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut moons = input.trim().lines().map(|l| {
        let mut s = l.split(",");
        let x = s.next().unwrap()[3..].parse::<isize>().unwrap();
        let y = s.next().unwrap()[3..].parse::<isize>().unwrap();
        let z = &s.next().unwrap()[3..];
        let z = z[0..z.len()-1].parse::<isize>().unwrap();
        ((x,y,z), (0isize, 0isize, 0isize))
    })
    .collect::<Vec<_>>();
    let init = moons.clone();

    let mut cx = 0usize;
    let mut cy = 0usize;
    let mut cz = 0usize;

    loop {
        stepx(&mut moons);
        cx += 1;
        if moons == init { break; }
    }
    moons = init.clone();
    loop {
        stepy(&mut moons);
        cy += 1;
        if moons == init { break; }
    }
    moons = init.clone();
    loop {
        stepz(&mut moons);
        cz += 1;
        if moons == init { break; }
    }

    let mut n = cx*cy / gcd(cx, cy);
    n = n * cz / gcd(n, cz);
    n
}

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}
