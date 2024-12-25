use std::{collections::*};
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut w = HashMap::new();

    let (init, expr) = input.split_once("\n\n").unwrap();

    for l in init.lines() {
        w.insert(&l[..3], l[5..].parse::<u8>().unwrap());
    }

    let mut changed = false;
    loop {
        changed = false;
        for e in expr.lines() {
            let [a, op, b, _, out] = *e.split_whitespace().cv() else { panic!() };
            if !w.contains_key(a) || !w.contains_key(b) {
                continue;
            }
            let a = w[a] != 0;
            let b = w[b] != 0;
            let val = match op {
                "AND" => a && b,
                "OR" => a || b,
                "XOR" => a ^ b,
                _ => unreachable!(),
            };
            if w.get(out).copied() == Some((!val) as u8) {
                panic!("wtf");
            }
            if !w.contains_key(out) {
                w.insert(out, val as u8);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut out = 0usize;
    for (k, v) in w.iter().sorted_unstable().rev() {
        if k.as_bytes()[0] == b'z' {
            out <<= 1;
            out |= *v as usize;
        }
    }
    format!("{out}")
}



pub fn uhh(input: &str) -> impl std::fmt::Display {
    let mut w = HashMap::new();

    let (init, expr) = input.split_once("\n\n").unwrap();

    for l in init.lines() {
        w.insert(&l[..3], l[5..].parse::<u8>().unwrap());
    }

    let mut changed = false;
    loop {
        changed = false;
        for e in expr.lines() {
            let [a, op, b, _, out] = *e.split_whitespace().cv() else { panic!() };
            if !w.contains_key(a) || !w.contains_key(b) {
                continue;
            }
            let a = w[a] != 0;
            let b = w[b] != 0;
            let val = match op {
                "AND" => a && b,
                "OR" => a || b,
                "XOR" => a ^ b,
                _ => unreachable!(),
            };
            if w.get(out).copied() == Some((!val) as u8) {
                panic!("wtf");
            }
            if !w.contains_key(out) {
                w.insert(out, val as u8);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    dbg!(w.len());

    for (&k, &v) in &w {
        if !k.starts_with(['x', 'y']) && v != 0 {
            eprintln!("{k}");
        }
    }

    let mut out = 0usize;
    for (k, v) in w.iter().sorted_unstable().rev() {
        if k.as_bytes()[0] == b'z' {
            out <<= 1;
            out |= *v as usize;
        }
    }
    // format!("{out:b}");
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut w = HashMap::new();

    let (init, expr) = input.split_once("\n\n").unwrap();

    for l in init.lines() {
        w.insert(&l[..3], l[5..].parse::<u8>().unwrap());
    }
    let mut exprs = expr.lines().map(|l| {
        let [mut a, op, mut b, _, out] = *l.split_whitespace().cv() else { panic!() };
        ((a.to_owned(), op, b.to_owned()), out)
    }).collect::<HashMap<_, _>>();
    let mut wire_labels = HashMap::<String, HashSet<String>>::new();
    let mut label = |wire, label| {
        wire_labels.entry(wire).or_default().insert(label);
    };

    let mut carry = None;
    for i in 0..45 {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let zf = format!("z{i:02}");
        if i == 0 {
            let z00 = exprs.get(&(x.clone(), "XOR", y.clone())).or(exprs.get(&(y.clone(), "XOR", x.clone()))).copied();
            if let Some(z00) = z00 {
                label(z00.to_owned(), "z00".into());
            }
            carry = exprs.get(&(x.clone(), "AND", y.clone())).or(exprs.get(&(y.clone(), "AND", x.clone()))).copied();
            if let Some(carry) = carry {
                label(carry.to_owned(), "c00".into());
            }
        } else {
            let i1 = exprs.get(&(x.clone(), "XOR", y.clone())).or(exprs.get(&(y.clone(), "XOR", x.clone()))).copied().unwrap();
            label(i1.to_owned(), format!("i1_{i:02}"));
            let subc = exprs.get(&(x.clone(), "AND", y.clone())).or(exprs.get(&(y.clone(), "AND", x.clone()))).copied().unwrap();
            label(subc.to_owned(), format!("subc_{i:02}"));

            if let Some(c) = carry {
                let c = c.to_owned();
                let i1 = i1.to_owned();
                let z = exprs.get(&(i1.clone(), "XOR", c.clone())).or(exprs.get(&(c.clone(), "XOR", i1.clone()))).copied();
                if let Some(z) = z {
                    label(z.to_owned(), format!("z{i:02}"));
                } else {
                    panic!("{i1:?} or {c:?} is sus, couldn't find {zf}");
                }
                let subc2 = exprs.get(&(i1.clone(), "AND", c.clone())).or(exprs.get(&(c.clone(), "AND", i1.clone()))).copied();
                if let Some(subc2) = subc2 {
                    label(subc2.to_owned(), format!("subc2_{i:02}"));
                    let subc = subc.to_owned();
                    let subc2 = subc2.to_owned();
                    carry = exprs.get(&(subc.clone(), "OR", subc2.clone())).or(exprs.get(&(subc2.clone(), "OR", subc.clone()))).copied();
                    if let Some(carry) = carry {
                        label(subc.to_owned(), format!("c{i:02}"));
                    } else {
                        panic!("{subc:?} or {subc2:?} is sus, couldn't find {zf}");
                    }
                } else {
                    panic!("{i1:?} or {c:?} is sus, couldn't find {zf}");
                }
            }

            // exprs.iter().filter(|e| *e.1 == &z).for_each(|(k, v)| {
            //     wire_labelsk.0
            // });
        }
    }

    for (w, l) in &wire_labels {
        // eprintln!("{w}: {l:?}");
    }

    // loop {
    //     let mut changed = false;
    //     for e in expr.lines() {
    //         let [a, op, b, _, out] = *e.split_whitespace().cv() else { panic!() };
    //         if !w.contains_key(a) || !w.contains_key(b) {
    //             continue;
    //         }
    //         let a = w[a] != 0;
    //         let b = w[b] != 0;
    //         let val = match op {
    //             "AND" => a && b,
    //             "OR" => a || b,
    //             "XOR" => a ^ b,
    //             _ => unreachable!(),
    //         };
    //         if w.get(out).copied() == Some((!val) as u8) {
    //             panic!("wtf");
    //         }
    //         if !w.contains_key(out) {
    //             w.insert(out, val as u8);
    //             changed = true;
    //         }
    //     }
    //     if !changed {
    //         break;
    //     }
    // }
    //
    // let mut x = 0usize;
    // let mut y = 0usize;
    // for (k, v) in w.iter().sorted_unstable().rev() {
    //     if k.as_bytes()[0] == b'x' {
    //         x <<= 1;
    //         x |= *v as usize;
    //     } else if k.as_bytes()[0] == b'y' {
    //         y <<= 1;
    //         y |= *v as usize;
    //     }
    // }
    //
    // let z = x+y;



    0
}
