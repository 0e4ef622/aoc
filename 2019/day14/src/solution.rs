use std::collections::*;
type R<'a, 'b: 'a> = &'a HashMap<&'b str, (usize, Vec<(usize, &'b str)>)>;

pub fn part1(input: &str) -> usize {
    let mut reactions = HashMap::new();

    for line in input.trim().lines() {
        let mut s = line.split(" => ");

        let r = s.next().unwrap();
        let rs = r.split(", ").map(|s| {
            let mut s = s.split(' ');
            let q = s.next().unwrap().parse::<usize>().unwrap();
            let n = s.next().unwrap();
            (q, n)
        }).collect::<Vec<_>>();

        let p = s.next().unwrap();
        let mut ps = p.split(' ');

        let pq = ps.next().unwrap().parse::<usize>().unwrap();
        let pn = ps.next().unwrap();

        reactions.insert(pn, (pq, rs));
    }

    let mut ingr: HashMap<_,_> = reactions.keys().copied().map(|x| (x, 0)).collect();
    ingr.insert("ORE", usize::max_value());
    satisfy(&reactions, &mut ingr, "FUEL", 1);
    usize::max_value() - ingr["ORE"]
}

fn satisfy(reactions: R, ingr: &mut HashMap<&str, usize>, n: &str, q: usize) -> bool {
    if ingr[n] >= q {
        *ingr.get_mut(n).unwrap() -= q;
        return true;
    }

    if n == "ORE" && ingr[n] < q { return false; }

    let rq = q - ingr[n];

    for p in &reactions[n].1 {
        if !satisfy(reactions, ingr, p.1, (1+(rq-1)/reactions[n].0) * p.0) { return false; }
    }
    *ingr.get_mut(n).unwrap() = ingr[n] + (1+(rq-1)/reactions[n].0)*reactions[n].0 - q;
    return true;
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut reactions = HashMap::new();

    for line in input.trim().lines() {
        let mut s = line.split(" => ");

        let r = s.next().unwrap();
        let rs = r.split(", ").map(|s| {
            let mut s = s.split(' ');
            let q = s.next().unwrap().parse::<usize>().unwrap();
            let n = s.next().unwrap();
            (q, n)
        }).collect::<Vec<_>>();

        let p = s.next().unwrap();
        let mut ps = p.split(' ');

        let pq = ps.next().unwrap().parse::<usize>().unwrap();
        let pn = ps.next().unwrap();

        reactions.insert(pn, (pq, rs));
    }

    let ore = 1000000000000;
    let mut f = 0;

    let mut ingr: HashMap<_,_> = reactions.keys().copied().map(|x| (x, 0)).collect();
    ingr.insert("ORE", ore);

    let mut i = 1;
    while i>0 {
        let mut clone = ingr.clone();
        if satisfy(&reactions, &mut clone, "FUEL", i) {
            f += i;
            ingr = clone;
            i *= 2;
        } else {
            i /= 2;
        }
    }

    f
}
