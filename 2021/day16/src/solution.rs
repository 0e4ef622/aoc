use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Debug)]
enum PktInner {
    Lit(Vec<u8>),
    Op(Vec<Pkt>),
}

#[derive(Debug)]
struct Pkt {
    version: u8,
    ty: u8,
    inner: PktInner,
}

fn vsum(p: &Pkt) -> usize {
    let mut sum = p.version as usize;
    match &p.inner {
        PktInner::Op(v) => {
            for p in v {
                sum += vsum(p);
            }
        }
        _ => (),
    }
    sum
}

fn calc(p: &Pkt) -> usize {
    match &p.inner {
        PktInner::Lit(v) => bn(v),
        PktInner::Op(v) => match p.ty {
            0 => v.iter().map(|p| calc(p)).sum::<usize>(),
            1 => v.iter().map(|p| calc(p)).product::<usize>(),
            2 => v.iter().map(|p| calc(p)).min().unwrap(),
            3 => v.iter().map(|p| calc(p)).max().unwrap(),
            5 => {
                let p1 = calc(&v[0]);
                let p2 = calc(&v[1]);
                (p1 > p2) as usize
            }
            6 => {
                let p1 = calc(&v[0]);
                let p2 = calc(&v[1]);
                (p1 < p2) as usize
            }
            7 => {
                let p1 = calc(&v[0]);
                let p2 = calc(&v[1]);
                (p1 == p2) as usize
            }
            _ => unreachable!(),
        }
    }
}

fn l(b: &mut &[u8]) -> Vec<u8> {
    let mut s = vec![];
    loop {
        let x = &b[..5]; *b = &b[5..];
        s.extend_from_slice(&x[1..]);
        if x[0] == 0 { break; }
    }
    s
}

fn bn(b: &[u8]) -> usize {
    let mut n = 0;
    for &d in b {
        n = n*2 + d as usize;
    }
    n
}

fn p(b: &mut &[u8]) -> Pkt {
    let version = bn(&b[..3]) as u8;
    *b = &b[3..];
    let ty = bn(&b[..3]) as u8;
    *b = &b[3..];

    if ty == 4 {
        let v = l(b);
        Pkt {
            version,
            ty,
            inner: PktInner::Lit(v),
        }
    } else {
        let tid = b[0];
        *b = &b[1..];
        if tid == 0 {
            let blen = bn(&b[..15]);
            *b = &b[15..];
            let mut sub = &b[..blen];
            *b = &b[blen..];
            let mut sp = vec![];
            while sub.len() > 0 {
                sp.push(p(&mut sub));
            }
            Pkt {
                ty,
                version,
                inner: PktInner::Op(sp),
            }
        } else {
            let plen = bn(&b[..11]);
            *b = &b[11..];
            let mut sp = vec![];
            for _ in 0..plen {
                sp.push(p(b));
            }
            Pkt {
                version,
                ty,
                inner: PktInner::Op(sp),
            }
        }
    }
}

fn hx(s: &str) -> Vec<u8> {
    let mut v = vec![];
    for c in s.bytes() {
        v.extend_from_slice(match c {
            b'0' => b"0000",
            b'1' => b"0001",
            b'2' => b"0010",
            b'3' => b"0011",
            b'4' => b"0100",
            b'5' => b"0101",
            b'6' => b"0110",
            b'7' => b"0111",
            b'8' => b"1000",
            b'9' => b"1001",
            b'A' => b"1010",
            b'B' => b"1011",
            b'C' => b"1100",
            b'D' => b"1101",
            b'E' => b"1110",
            b'F' => b"1111",
            _ => unreachable!(),
        });
    }
    for b in &mut v { *b -= b'0'; }
    v
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let d = hx(input.trim());
    vsum(&p(&mut &d[..]))
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let d = hx(input.trim());
    calc(&p(&mut &d[..]))
}
