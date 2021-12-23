use std::convert::TryInto;
use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut c = HashMap::new();
    for line in input.lines() {
        let w = line.split(' ').cv();
        let r = w[1].split(',').flat_map(|s| s[2..].split("..")).map(|v| v.parse::<i64>().unwrap()).cv();
        for (x,y,z) in iproduct!(r[0].max(-50)..=r[1].min(50),r[2].max(-50)..=r[3].min(50),r[4].max(-50)..=r[5].min(50)) {
            c.insert((x,y,z), w[0]=="on");
        }
    }
    c.values().filter(|x|**x).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut q = vec![];
    for line in input.lines() {
        let w = line.split(' ').cv();
        let r = w[1].split(',').flat_map(|s| s[2..].split("..")).map(|v| v.parse::<i64>().unwrap()).cv();
        let r: Cube = r.try_into().unwrap();
        let mut r = r.app(|[a,b,c,d,e,f]|[a,c,e,b,d,f]);
        r[3..].iter_mut().for_each(|x| *x+=1);
        q.push((w[0], r));
    }
    dqx([-200000,-200000,-200000,200000,200000,200000], &q)
}

fn dqx(r: Cube, q: &[(&str, Cube)]) -> i64 {
    let mut q = q.to_vec();
    // dbg!(q.len());
    // eprintln!("{}..{}", r[0], r[3]);
    if q.iter().all(|(_,c)| c[0]==r[0]&&c[3]==r[3]) { return dqy(r, &q) }

    let mut qq = q.to_vec();

    let m = (r[0]+r[3])/2;
    for v in &mut qq { v.1[3] = v.1[3].min(m); }
    qq.retain(|(_,c)| c[0]<c[3]);
    let l = dqx(r.ch(|r| r[3]=m), &qq);
    qq = q.to_vec();
    for v in &mut qq { v.1[0] = v.1[0].max(m); }
    qq.retain(|(_,c)| c[0]<c[3]);
    let r = dqx(r.ch(|r| r[0]=m), &qq);
    l+r
}
fn dqy(r: Cube, q: &[(&str, Cube)]) -> i64 {
    if q.iter().all(|(_,c)| c[1]==r[1]&&c[4]==r[4]) { return dqz(r, q) }
    let mut qq = q.to_vec();
    let m = (r[1]+r[4])/2;
    for v in &mut qq { v.1[4] = v.1[4].min(m); }
    qq.retain(|(_,c)| c[1]<c[4]);
    let l = dqy(r.ch(|r| r[4]=m), &qq);
    qq = q.to_vec();
    for v in &mut qq { v.1[1] = v.1[1].max(m); }
    qq.retain(|(_,c)| c[1]<c[4]);
    let r = dqy(r.ch(|r| r[1]=m), &qq);
    l+r
}
fn dqz(r: Cube, q: &[(&str, Cube)]) -> i64 {
    let mut q = q.to_vec();
    if let Some((ix,_)) = q.iter().enumerate().rev().find(|(_,(_,c))| c[2]==r[2]&&c[5]==r[5]) {
        q.drain(..ix);
    }
    if q.iter().all(|(_,c)| c[2]==r[2]&&c[5]==r[5]) {
        if q.is_empty() || q.last().unwrap().0 == "off" {
            return 0;
        } else {
            return area(r);
        }
    }
    let mut qq = q.to_vec();
    let m = (r[2]+r[5])/2;
    for v in &mut qq { v.1[5] = v.1[5].min(m); }
    qq.retain(|(_,c)| c[2]<c[5]);
    let l = dqz(r.ch(|r| r[5]=m), &qq);
    qq = q.to_vec();
    for v in &mut qq { v.1[2] = v.1[2].max(m); }
    qq.retain(|(_,c)| c[2]<c[5]);
    let r = dqz(r.ch(|r| r[2]=m), &qq);
    l+r

}

fn ix([a,b,c,d,e,f]: Cube, [A,B,C,D,E,F]: Cube) -> i64 {
    let [a,b,c,d,e,f] = [a.max(A), b.max(B), c.max(C), d.min(D), e.min(E), f.min(F)];
    if a>d || b>e || c>f { 0 }
    else { area([a,b,c,d,e,f]) }

}
fn area([x,y,z,u,v,w]: Cube) -> i64 {
    -((x-u)*(y-v)*(z-w))
}
type Cube = [i64; 6];
